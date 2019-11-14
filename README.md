anondb
======

Naive implementation of a "secret triple store" using Enigma.
It enables a freeform, trustless, federated data bank that can be queried to answer questions about people and devices without otherwise compromising their privacy.

Examples
--------

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

Rationale
---------

RDF namespaces are a natural fit for a federated model of trust.
The bar above could optionally allow proving your age via a nongovernmental third party in case you don't have a license, or they could also require you to be a member of a particular organization such as a church or city, or *not* a member of an organization like the local Alcoholics Anonymous.

Unfortunately, triplestores don't scale well compared to traditional databases. However, neither do blockchains so the problem isn't immediate. In the long run, a more efficient data structure may be necessary.

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

Better name? sigdb, anondb, witsec, secret turtle, he-said-she-said...

Simpler example without 2fa?

Do queries have to be signed by the entities involved?

What about N-quads?

Is the unique human identity thing really required?
Where does the DMV's signature get encoded?

Is it possible to execute an arbitrary function/program as a predicate?
Something like <person1> nebula:cousin <person2>.

[foaf]: ???
[hd]: ???
