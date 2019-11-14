anondb
======

Naive implementation of a "secret triple store" on [Enigma][enigma].
It creates a trustless, freeform, federated data bank for proving assertions about yourself without revealing your identity.

This is just proof-of-concept code. However, it should be fairly reliable since the complicated parts map onto existing RDF concepts, and RDF can be validated using the [W3C test site][w3c] (basic) and/or ontology theorum provers (advanced).

The main caveats is that it may need to be optimized to scale to a large number of triples.

Concept
-------

Any party can add self-signed statements concerning a person to the data bank's encrypted secret contract state, such as:

* <person> controls the private key corresponding to <pubkey> (this is self-evident once signed)
* <person> owns the 2FA device <deviceid>
* <person>'s social security number is <ssn>
* <person>'s facial profile hash is <hash>
* <person>'s fingerprint hash is <hash>
* <person>'s gender is male
* <person>'s DNA haplotype is <hash>
* <person>'s birthday is Aug 29, 1975
* <person> is of legal drinking age
* <person> qualifies for food stamps through Dec 31, 2019
* <person> qualifies to participate the in the <token> ICO
* <person>'s Experian credit score as of Nov 14, 2019 is 750

Later, the person can sign and execute queries to prove assertions about themself to another party without revealing their identity.

The assertions can be simple, such as "I am of legal US drinking age" or "I am on the whitelist". Or they can be derived from multiple independent sources, which may refer to each other. For example, "I qualify for a $5000 loan fom your bank" might expand to "I am a US citizen, and I am over 18 years old, and I am not a felon, and I have a credit score of at least 700" and be verified using statements from the police + DMV + credit rating agencies.

Although one `<person>` variable is shown above for simplicity, no global identity known to all parties is required. Instead the queries work more like real life identification: "I am the person with access to my phone, whose picture is shown on my passport and/or drivers license, and whose SSN I know". Signing parties can define identity however they want, and queries can mix and match statements from multiple parties.

Examples
--------

Suppose you want to prove that you can 

You could prove your legal drinking age at a bar without showing the bouncer your drivers' license as follows:

1. A trusted third party, such as the DMV or a public notary, checks your license and sees that your birthday is Aug 29, 1975. They also require you to confirm your unique human identity, perhaps via a blockchain system like [HumanityDAO][hd], a government-issued 2FA device showing your picture, or biometric data such as your fingerprint or facial structure.

2. They sign two triples (subject-predicate-object statements) to the effect that you are the person corresponding to the ID, and what it says your birthday is. For the 2fa device, something like:

```
<witness>: <person> dmv:2fa <id>
<witness>: <person> dmv:birthday "1975-08-29"
```

"dmv" here is an alias for the Department of Motor Vehicles namespace, which would include the predicates `2fa` and `birthday`.

3. Later, the bar checks that you look like the picture on the 2fa device and requires you to execute a secret contract querying anondb, which will return whether the person corresponding to that device is over 21 today:

```
exists ?anon where:

<dmv>: <notary> notary:public valid.
<dmv>: ?anon dmv:2fa <id>.
<notary>: ?anon dmv:2fa <id>.
<notary>: ?anon dmv:birthday <= "1998-11-13".
```

No unneccesary information is exchanged! The bar gets proof of your age without needing to know your name, birthday, or even home state. They only need to know that you look like your picture and are over 21 according to some valid signatory.

The same system could be extended to work without the DMV by trusting a public notary or corporation to verify IDs.

Format
------

Data is entered in standard RDF format (turtle, n-triples, or RDF XML), except:

* Namespaces used for signatures must include a public key
* Statements must be signed with the corresponding private key

So for example the California DMV might produce data like this as part of issuing a "RealID":

```
-----BEGIN PGP SIGNED MESSAGE-----
Hash: SHA512

dmv: <https://dmv.ca.gov/81AA29BF8170AECCB0AD2C3D326F1782DA67E42B>

<jdoe> 	a dmv:person;
	dmv:fullname "Jane Doe";
	dmv:birthday "1975-08-29";
	dmv:realid <id>.
-----BEGIN PGP SIGNATURE-----

iQGzBAEBCgAdFiEEgaopv4FwrsywrSw9Mm8Xgtpn5CsFAl3NktEACgkQMm8Xgtpn
5Csjjwv+Pofi5O/Hc61Enw0KMK2sPlavJ8AjNkt7Wlf8TPvrRBZCOfHBNKNtttnu
/FQ0cWBQIotx/CmqoySzc2KPOErUXBshhb54ckpwLh7D4/oVrqc3agRRZijnFgO/
z48oA54svL3Pjz/R7f7jpBk2Bh9DimUURtP/wyFlZ1c/2Z3LFN28Zvc2osTSIfzy
VoE3EP5g4LqFeRyadLUU35yEBrccoj+irtMtxzY+ZKR7j+wdXOY941OdzBU0y9iC
trFJA4EGEsqWr2uNwYgqD4fFXugMOuw177LeNhm7++D+Dds/XztuKnlWWc4HVHJT
+6/35KgJSf3QgHxRDGN0OephVYQmgaHqjQeig5BCfDF6C3QUpXaya8lVDM0kZCoB
Ljn5mmoYzdu11HxouhL7itLuITJmEBIbWKnTEsls8d2MeLEfrxtS0czhLS2PkrUf
XLAQsdcYT+5H0todcrKbeasNF97lIxmnPcQb4/b4Bz5jL6i4y+isy764YaE6VNsg
mm73Wp8G
=ymGl
-----END PGP SIGNATURE-----
```

Of course, the `<jdoe>` and `<id>` URIs would be filled in.
The predicates would be defined in a separate California DMV ontology signed with the same key.
`person` and `fullname` would probably be defined as matching their standard [foaf][foaf] equivalents,
while `realid` would probably be defined in terms of another ontology signed by the federal government.
`birthday` could go either way.

TODO
----

How should signatures be used? Do they go in the predicates or separate?

Better name? sigdb, anondb, witsec, secret turtle, he-said-she-said, assert(ive)...

Does this relate to/integrate with keybase?

Simpler example: DMV verfies things itself first?

Do queries have to be signed by the entities involved?

What about N-quads?

Is the unique human identity thing really required?
Where does the DMV's signature get encoded?

Is it possible to execute an arbitrary function/program as a predicate?
Something like <person1> nebula:cousin <person2>.

[enigma]: https://enigma.co
[foaf]: ???
[hd]: ???
[w3c]: ???
