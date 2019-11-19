anondb
======

A trustless, freeform, federated data bank for proving assertions about yourself without revealing your identity.

Implemented as a naive "secret graph store" for [Enigma][enigma].
It should be fairly reliable since the complicated parts map onto existing RDF concepts,
and RDF can be validated using the [W3C test site][w3c] (basic) and/or ontology theorum provers (advanced).
The main caveat is that it may need to be optimized to scale to a large number of triples.

Concept
-------

Any party can add self-signed statements concerning people to the data bank's encrypted secret contract state, such as:

* <anon> has the private key for <pubkey> (this one is uniquely self-evident)
* <federal-gov> says <anon>'s social security number is <ssn>
* <state-gov> says <anon>'s gender is male
* <state-gov> says <anon>'s birthday is Aug 29, 1975
* <state-gov> says <anon> is of legal drinking age
* <state-gov> says <anon> qualifies for food stamps through Dec 31, 2019
* <2fa-company> says <anon> owns the 2FA device <deviceid>
* <biometrics-company> says <anon>'s facial profile hash is <hash>
* <biometrics-company> says <anon>'s fingerprint hash is <hash>
* <sequencing-company> says <anon>'s DNA haplotype is <hash>
* <kyc-company> says <anon> qualifies to participate the in the <token> ICO
* <credit-company> says <anon>'s credit score as of Nov 14, 2019 is 750

Later, `<anon>` can sign and execute queries to prove those assertions about themself to another party without revealing their identity.

The assertions can be simple, such as "<anon> is of legal drinking age in <state>" or "<anon> is on the <token> whitelist". Or they can be derived from multiple independent sources, which may refer to each other. For example, "<anon> qualifies for a $5000 loan from <bank>" might expand to "<anon> is a US citizen, and <anon> is over 18 years old, and <anon> is not a felon, and <anon> has a credit score of at least 750" and be verified using separate statements from the police + DMV + credit rating agencies.

Although one `<anon>` variable is shown above for simplicity, no global identity known to all parties is required! Instead the queries work more like real life identification: "I am the person with access to my phone, whose picture is shown on my passport and/or drivers license, and whose SSN I know". Queries can mix and match statements from multiple signing parties, and each party may require different forms of identification.

Data entry
----------

Data can be entered in any standard RDF format, except that:

* the default graph is reserved for internal use
* a separate named graph is added per signing party, named by their public key
* triples (quads really) must be signed by the corresponding private key

So for example the California DMV might produce data like this as part of issuing a "RealID":

TODO fill in example

Queries
-------

TODO fill in example

[enigma]: https://enigma.co
[enigmajs]: ???
[foaf]: ???
[hd]: ???
[w3c]: ???
