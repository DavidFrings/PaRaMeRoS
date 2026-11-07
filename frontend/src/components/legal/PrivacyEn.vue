<script setup lang="ts">
import { useLocale } from '@/composables/useLocale'

const { localePath } = useLocale()
</script>

<template>
  <div>
    <p class="font-medium">The German version is legally binding.</p>
    <h2 id="1">Preamble</h2>
    <p>
      With the following privacy policy we would like to inform you which types of your personal
      data (hereinafter also referred to as "data") we process, for what purposes and to what
      extent. The privacy policy applies to all processing of personal data carried out by us, both
      in the course of providing our services and in particular on our websites, in mobile
      applications and within external online presences, such as our social media profiles
      (hereinafter collectively referred to as "online offering").
    </p>
    <p>The terms used are not gender-specific.</p>

    <p>Last updated: 7 November 2026</p>
    <h2>Table of contents</h2>
    <ul>
      <li><a href="#1">Preamble</a></li>
      <li><a href="#2">Controller</a></li>
      <li><a href="#3">Overview of processing operations</a></li>
      <li><a href="#4">Relevant legal bases</a></li>
      <li><a href="#5">Security measures</a></li>
      <li><a href="#6">Transmission of personal data</a></li>
      <li><a href="#7">General information on data storage and deletion</a></li>
      <li><a href="#8">Rights of data subjects</a></li>
      <li><a href="#9">Provision of the online offering and web hosting</a></li>
      <li><a href="#10">Use of cookies</a></li>
      <li><a href="#11">Blogs and publication media</a></li>
      <li><a href="#12">Contact and enquiry management</a></li>
      <li><a href="#13">Plug-ins and embedded functions and content</a></li>
      <li><a href="#14">Changes and updates</a></li>
    </ul>
    <h2 id="2">Controller</h2>
    <p>
      Erzbischöfliche Pater-Rupert-Mayer Realschule<br />Wolfratshauserstr. 30<br />82049 Pullach im
      Isartal
    </p>
    <p>Represented by: Markus v. Essen</p>
    <p>Email address: parameros@mein.gmx</p>
    <p>Phone: 089/74426-182</p>
    <p>Legal notice: <RouterLink :to="localePath('/Imprint')">Legal notice</RouterLink></p>

    <h2 id="3">Overview of processing operations</h2>
    <p>
      The following overview summarises the types of data processed and the purposes of their
      processing and refers to the data subjects concerned.
    </p>
    <h3>Types of data processed</h3>
    <ul>
      <li>Master data.</li>
      <li>Contact data.</li>
      <li>Content data.</li>
      <li>Usage data.</li>
      <li>Meta, communication and procedural data.</li>
      <li>Log data.</li>
    </ul>
    <h3>Categories of data subjects</h3>
    <ul>
      <li>Communication partners.</li>
      <li>Users.</li>
    </ul>
    <h3>Purposes of processing</h3>
    <ul>
      <li>Communication.</li>
      <li>Security measures.</li>
      <li>Organisational and administrative procedures.</li>
      <li>Feedback.</li>
      <li>Provision of our online offering and user-friendliness.</li>
      <li>Information technology infrastructure.</li>
    </ul>
    <h2 id="4">Relevant legal bases</h2>

    <h3>Relevant legal bases under the GDPR:</h3>
    <p>
      Below you will find an overview of the legal bases of the GDPR on which we process personal
      data. Please note that, in addition to the provisions of the GDPR, national data protection
      rules may apply in your country or in ours. Should more specific legal bases also apply in an
      individual case, we will inform you of them in the privacy policy.
    </p>
    <ul>
      <li>
        <b>Consent (Art. 6 (1) sentence 1 lit. a) GDPR)</b> - The data subject has given consent to
        the processing of personal data concerning them for a specific purpose or several specific
        purposes.
      </li>
      <li>
        <b
          >Performance of a contract and pre-contractual enquiries (Art. 6 (1) sentence 1 lit. b)
          GDPR)</b
        >
        - Processing is necessary for the performance of a contract to which the data subject is a
        party or for taking steps prior to entering into a contract at the request of the data
        subject.
      </li>
      <li>
        <b>Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR)</b> - Processing is necessary
        for the purposes of the legitimate interests pursued by the controller or by a third party,
        except where such interests are overridden by the interests or fundamental rights and
        freedoms of the data subject which require protection of personal data.
      </li>
    </ul>

    <h3>National data protection rules in Germany:</h3>
    <p>
      In addition to the data protection rules of the GDPR, national data protection rules apply in
      Germany. These include in particular the Federal Data Protection Act (Bundesdatenschutzgesetz
      – BDSG). The BDSG contains in particular special rules on the right of access, the right to
      erasure, the right to object, the processing of special categories of personal data,
      processing for other purposes and transmission, as well as automated individual
      decision-making including profiling. Furthermore, the data protection laws of the individual
      German states may apply.
    </p>

    <h3>Note on the applicability of the GDPR and the Swiss FADP:</h3>
    <p>
      This privacy information serves to provide information both under the Swiss Federal Act on
      Data Protection (FADP) and under the General Data Protection Regulation (GDPR). For this
      reason, please note that, because of their broader geographical application and
      comprehensibility, the terms of the GDPR are used. In particular, instead of the terms used in
      the Swiss FADP, such as "processing" of "personal data", "overriding interest" and "sensitive
      personal data", the terms used in the GDPR, "processing" of "personal data", "legitimate
      interest" and "special categories of data", are used. The legal meaning of the terms will
      however continue to be determined by the Swiss FADP within the scope of its application.
    </p>

    <h2 id="5">Security measures</h2>
    <p>
      In accordance with the legal requirements, taking into account the state of the art, the costs
      of implementation and the nature, scope, context and purposes of processing, as well as the
      varying likelihood and severity of the risk to the rights and freedoms of natural persons, we
      implement appropriate technical and organisational measures to ensure a level of security
      appropriate to the risk.
    </p>
    <p>
      The measures include in particular safeguarding the confidentiality, integrity and
      availability of data by controlling physical and electronic access to the data as well as
      access to it, input, disclosure, ensuring availability and separation. Furthermore, we have
      established procedures to ensure the exercise of data subject rights, the deletion of data and
      responses to threats to the data. We also take the protection of personal data into account
      already in the development and selection of hardware, software and procedures, in accordance
      with the principle of data protection by design and by default.
    </p>
    <p>
      Securing online connections with TLS/SSL encryption technology (HTTPS): To protect the data of
      users transmitted via our online services from unauthorised access, we rely on TLS/SSL
      encryption technology. Secure Sockets Layer (SSL) and Transport Layer Security (TLS) are the
      cornerstones of secure data transmission on the internet. These technologies encrypt the
      information transmitted between the website or app and the user's browser (or between two
      servers), protecting the data from unauthorised access. TLS, as the more advanced and more
      secure version of SSL, ensures that all data transmissions meet the highest security
      standards. If a website is secured by an SSL/TLS certificate, this is indicated by the display
      of HTTPS in the URL. This serves as an indicator for users that their data is transmitted
      securely and encrypted.
    </p>

    <h2 id="6">Transmission of personal data</h2>
    <p>
      In the course of our processing of personal data, it happens that data is transmitted to or
      disclosed to other bodies, companies, legally independent organisational units or persons.
      Recipients of this data may include, for example, service providers commissioned with IT tasks
      or providers of services and content that is embedded in a website. In such cases, we observe
      the legal requirements and in particular conclude corresponding contracts or agreements that
      serve to protect your data with the recipients of your data.
    </p>
    <p>
      Data transmission within the organisation: We may transmit personal data to other departments
      or units within our organisation or grant them access to it. Where data is passed on for
      administrative purposes, this is based on our legitimate business and commercial interests or
      takes place where it is necessary for the performance of our contractual obligations or where
      the data subjects have given consent or there is legal permission.
    </p>

    <h2 id="7">General information on data storage and deletion</h2>
    <p>
      We delete personal data that we process in accordance with the statutory provisions as soon as
      the underlying consents are withdrawn or there are no further legal grounds for the
      processing. This applies in cases where the original purpose of processing no longer applies
      or the data is no longer needed. Exceptions to this rule apply where statutory obligations or
      special interests require longer retention or archiving of the data.
    </p>
    <p>
      In particular, data that must be retained for commercial or tax law reasons, or whose storage
      is necessary for legal prosecution or to protect the rights of other natural or legal persons,
      must be archived accordingly.
    </p>
    <p>
      Our privacy information contains additional information on the retention and deletion of data
      that applies specifically to certain processing operations.
    </p>
    <p>
      Where several retention periods or deletion deadlines are specified for a data item, the
      longest period always applies. Data that is no longer retained for its originally intended
      purpose but due to legal requirements or other reasons is processed by us exclusively for the
      reasons that justify its retention.
    </p>
    <p>
      Retention and deletion of data: The following general periods apply to retention and archiving
      under German law:
    </p>
    <ul>
      <li>
        10 years - Retention period for books and records, annual financial statements, inventories,
        management reports, opening balance sheets and the work instructions and other
        organisational documents required to understand them (Section 147 (1) no. 1 in conjunction
        with (3) AO, Section 14b (1) UStG, Section 257 (1) no. 1 in conjunction with (4) HGB).
      </li>
      <li>
        8 years - Accounting vouchers, such as invoices and expense receipts (Section 147 (1) nos. 4
        and 4a in conjunction with (3) sentence 1 AO and Section 257 (1) no. 4 in conjunction with
        (4) HGB).
      </li>
      <li>
        6 years - Other business documents: received commercial or business letters, copies of sent
        commercial or business letters, other documents insofar as they are relevant for taxation,
        e.g. hourly wage slips, operating accounting sheets, calculation documents, price tags, but
        also payroll documents insofar as they are not already accounting vouchers, and cash
        register receipts (Section 147 (1) nos. 2, 3, 5 in conjunction with (3) AO, Section 257 (1)
        nos. 2 and 3 in conjunction with (4) HGB).
      </li>
      <li>
        3 years - Data required to take potential warranty and damages claims or similar contractual
        claims and rights into account and to process related enquiries, based on previous business
        experience and customary industry practice, is stored for the duration of the regular
        statutory limitation period of three years (Sections 195, 199 BGB).
      </li>
    </ul>
    <h2 id="8">Rights of data subjects</h2>
    <p>
      Rights of data subjects under the GDPR: As a data subject, you have various rights under the
      GDPR, which arise in particular from Art. 15 to 21 GDPR:
    </p>
    <ul>
      <li>
        <b
          >Right to object: You have the right to object at any time, on grounds relating to your
          particular situation, to the processing of personal data concerning you which is based on
          Art. 6 (1) lit. e or f GDPR; this also applies to profiling based on these provisions. If
          the personal data concerning you is processed for direct marketing purposes, you have the
          right to object at any time to the processing of personal data concerning you for the
          purpose of such marketing; this also applies to profiling to the extent that it is related
          to such direct marketing.</b
        >
      </li>
      <li>
        <b>Right to withdraw consent:</b> You have the right to withdraw consent given at any time.
      </li>
      <li>
        <b>Right of access:</b> You have the right to request confirmation as to whether data
        concerning you is being processed and to request access to this data as well as further
        information and a copy of the data in accordance with the legal requirements.
      </li>
      <li>
        <b>Right to rectification:</b> In accordance with the legal requirements, you have the right
        to request the completion of data concerning you or the rectification of inaccurate data
        concerning you.
      </li>
      <li>
        <b>Right to erasure and restriction of processing:</b> In accordance with the legal
        requirements, you have the right to request that data concerning you be erased without undue
        delay or, alternatively, to request restriction of the processing of the data in accordance
        with the legal requirements.
      </li>
      <li>
        <b>Right to data portability:</b> You have the right to receive data concerning you which
        you have provided to us in a structured, commonly used and machine-readable format in
        accordance with the legal requirements, or to request its transmission to another
        controller.
      </li>
      <li>
        <b>Complaint to a supervisory authority:</b> Without prejudice to any other administrative
        or judicial remedy, you have the right to lodge a complaint with a supervisory authority, in
        particular in the Member State of your habitual residence, place of work or place of the
        alleged infringement, if you consider that the processing of personal data concerning you
        infringes the GDPR.
      </li>
    </ul>

    <h2 id="9">Provision of the online offering and web hosting</h2>
    <p>
      We process users' data in order to be able to provide them with our online services. For this
      purpose, we process the user's IP address, which is necessary to transmit the content and
      functions of our online services to the user's browser or device.
    </p>
    <ul>
      <li>
        <b>Types of data processed:</b> Usage data (e.g. page views and dwell time, click paths,
        intensity and frequency of use, device types and operating systems used, interactions with
        content and functions); meta, communication and procedural data (e.g. IP addresses,
        timestamps, identification numbers, persons involved). Log data (e.g. log files concerning
        logins or the retrieval of data or access times.).
      </li>
      <li><b>Data subjects:</b> Users (e.g. website visitors, users of online services).</li>
      <li>
        <b>Purposes of processing:</b> Provision of our online offering and user-friendliness;
        information technology infrastructure (operation and provision of information systems and
        technical devices (computers, servers, etc.)). Security measures.
      </li>
      <li>
        <b>Retention and deletion:</b> Deletion in accordance with the information in the section
        "General information on data storage and deletion".
      </li>
      <li><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</li>
    </ul>
    <h3>Further information on processing operations, procedures and services:</h3>
    <ul>
      <li>
        <b>Provision of the online offering on rented storage space: </b>To provide our online
        offering, we use storage space, computing capacity and software that we rent or otherwise
        obtain from a corresponding server provider (also called "web host");
        <span><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</span>
      </li>
      <li>
        <b>Collection of access data and log files: </b>Access to our online offering is logged in
        the form of so-called "server log files". Server log files may include the address and name
        of the web pages and files accessed, date and time of access, data volumes transferred,
        notification of successful access, browser type and version, the user's operating system,
        referrer URL (the previously visited page) and, as a rule, IP addresses and the requesting
        provider. The server log files can be used for security purposes, e.g. to avoid overloading
        the servers (in particular in the case of abusive attacks, so-called DDoS attacks), and also
        to ensure the load on the servers and their stability;
        <span><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR). </span
        ><b>Deletion of data:</b> Log file information is stored for a maximum of 30 days and then
        deleted or anonymised. Data whose further retention is necessary for evidentiary purposes is
        exempt from deletion until the respective incident has been finally clarified.
      </li>
      <li>
        <b>Content delivery network: </b>We use a "content delivery network" (CDN). A CDN is a
        service that helps deliver content of an online offering, in particular large media files
        such as graphics or program scripts, faster and more securely with the help of regionally
        distributed servers connected via the internet;
        <span><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</span>
      </li>
      <li>
        <b>STRATO: </b>Services in the field of providing information technology infrastructure and
        related services (e.g. storage space and/or computing capacity);
        <b>Service provider:</b> STRATO AG, Pascalstraße 10, 10587 Berlin, Germany;
        <span><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR); </span
        ><b>Website: </b> <a href="https://www.strato.de" target="_blank">https://www.strato.de</a>;
        <b>Privacy policy: </b>
        <a href="https://www.strato.de/datenschutz/" target="_blank"
          >https://www.strato.de/datenschutz/</a
        >. <b>Data processing agreement:</b> Provided by the service provider.
      </li>
      <li>
        <b>Rate limiting with Redis: </b>To protect against abuse and overload (e.g. repeated login
        attempts), our server processes the user's IP address in a Redis cache in order to limit the
        number of requests per IP address. The IP address is stored there for at most 1 hour and
        then deleted automatically;
        <span><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</span>
      </li>
    </ul>
    <h2 id="10">Use of cookies</h2>
    <p>
      The term "cookies" refers to functions that store information on users' end devices and read
      it from them. Cookies can also be used for various purposes, such as the functionality,
      security and convenience of online offerings and the creation of analyses of visitor flows. We
      use cookies in accordance with the statutory provisions. To this end, we obtain users' consent
      in advance where required. Where consent is not necessary, we rely on our legitimate
      interests. This applies where the storage and reading of information is essential in order to
      provide expressly requested content and functions. This includes, for example, the storage of
      settings and ensuring the functionality and security of our online offering. Consent can be
      withdrawn at any time. We inform clearly about its scope and which cookies are used.
    </p>
    <p>
      <b>Notes on data protection legal bases: </b>Whether we process personal data using cookies
      depends on consent. If consent has been given, it serves as the legal basis. Without consent,
      we rely on our legitimate interests, which are explained above in this section and in the
      context of the respective services and procedures.
    </p>
    <p>
      <b>Storage duration: </b>With regard to the storage duration, the following types of cookies
      are distinguished:
    </p>
    <ul>
      <li>
        <b>Temporary cookies (also: session cookies):</b> Temporary cookies are deleted at the
        latest after a user has left an online offering and closed their end device (e.g. browser or
        mobile application).
      </li>
      <li>
        <b>Permanent cookies:</b> Permanent cookies remain stored even after the end device is
        closed. For example, the login status can be stored and preferred content can be displayed
        directly when the user visits a website again. Likewise, the user data collected using
        cookies can be used for reach measurement. Unless we give users explicit information on the
        type and storage duration of cookies (e.g. when obtaining consent), they should assume that
        cookies are permanent and that the storage duration can be up to two years.
      </li>
    </ul>
    <p>
      <b>General notes on withdrawal and objection (opt-out): </b>Users can withdraw the consent
      they have given at any time and also object to the processing in accordance with the legal
      requirements, including via the privacy settings of their browser.
    </p>
    <b>Cookie settings / option to object:</b>
    <p><a href="https://parameros.net/cookies" target="_blank">Cookies</a></p>
    <ul>
      <li>
        <b>Types of data processed:</b> Meta, communication and procedural data (e.g. IP addresses,
        timestamps, identification numbers, persons involved).
      </li>
      <li><b>Data subjects:</b> Users (e.g. website visitors, users of online services).</li>
      <li>
        <b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR). Consent (Art.
        6 (1) sentence 1 lit. a) GDPR).
      </li>
    </ul>
    <p><b>Further information on processing operations, procedures and services:</b></p>
    <ul>
      <li>
        <b>Processing of cookie data on the basis of consent: </b>We use a consent management
        solution in which users' consent to the use of cookies, or to the procedures and providers
        named in the consent management solution, is obtained. This procedure serves to obtain,
        record, manage and withdraw consent, in particular with regard to the use of cookies and
        comparable technologies used to store, read and process information on users' end devices.
        In this procedure, users' consent to the use of cookies and the associated processing of
        information, including the specific processing operations and providers named in the consent
        management procedure, is obtained. Users also have the option to manage and withdraw their
        consent. The declarations of consent are stored in order to avoid repeated queries and to be
        able to provide proof of consent as required by law. The storage takes place on the server
        side and/or in a cookie (so-called opt-in cookie) or by means of comparable technologies in
        order to be able to assign the consent to a specific user or their device. Unless specific
        information on the providers of consent management services is available, the following
        general notes apply: The consent is stored for up to two years. A pseudonymous user
        identifier is created, which is stored together with the time of consent, information on the
        scope of consent (e.g. relevant categories of cookies and/or service providers) and
        information on the browser, system and end device used;
        <span><b>Legal bases:</b> Consent (Art. 6 (1) sentence 1 lit. a) GDPR).</span>
      </li>
      <li>
        <b>Language setting (pm-lang): </b>If you choose German or English using the language
        switch, we store this choice under the key "pm-lang" in your browser's local storage
        (localStorage). The entry serves solely to keep your language on your next visit. It is
        functional and necessary for this purpose, is not transmitted to us or to third parties and
        can be deleted at any time via the browser settings;
        <span><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</span>
      </li>
    </ul>
    <h2 id="11">Blogs and publication media</h2>
    <p>
      We use blogs or comparable means of online communication and publication (hereinafter
      "publication medium"). Readers' data is processed for the purposes of the publication medium
      only to the extent necessary for its presentation and for communication between authors and
      readers or for security reasons. Otherwise, we refer to the information on the processing of
      visitors to our publication medium within this privacy information.
    </p>
    <ul>
      <li>
        <b>Types of data processed:</b> Master data (e.g. full name, home address, contact
        information, customer number, etc.); contact data (e.g. postal and email addresses or phone
        numbers); content data (e.g. text or image messages and posts and information relating to
        them, such as details of authorship or time of creation); usage data (e.g. page views and
        dwell time, click paths, intensity and frequency of use, device types and operating systems
        used, interactions with content and functions). Meta, communication and procedural data
        (e.g. IP addresses, timestamps, identification numbers, persons involved).
      </li>
      <li><b>Data subjects:</b> Users (e.g. website visitors, users of online services).</li>
      <li>
        <b>Purposes of processing:</b> Feedback (e.g. collecting feedback via online form);
        provision of our online offering and user-friendliness; security measures. Organisational
        and administrative procedures.
      </li>
      <li>
        <b>Retention and deletion:</b> Deletion in accordance with the information in the section
        "General information on data storage and deletion".
      </li>
      <li><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</li>
    </ul>
    <p><b>Further information on processing operations, procedures and services:</b></p>
    <ul>
      <li>
        <b>Comments and posts: </b>If users leave comments or other posts, their IP addresses may be
        stored on the basis of our legitimate interests. This is done for our security in case
        someone leaves unlawful content in comments and posts (insults, prohibited political
        propaganda, etc.). In this case, we ourselves can be held liable for the comment or post and
        are therefore interested in the identity of the author.<br />Furthermore, we reserve the
        right, on the basis of our legitimate interests, to process users' details for the purpose
        of spam detection.<br />On the same legal basis, we reserve the right, in the case of polls,
        to store users' IP addresses for their duration and to use cookies in order to prevent
        multiple voting.<br />The personal information, any contact and website information as well
        as the content provided in the context of comments and posts will be stored by us
        permanently until the users object;
        <span><b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</span>
      </li>
    </ul>
    <h2 id="12">Contact and enquiry management</h2>
    <p>
      When contacting us (e.g. by post, contact form, email, phone or via social media) and in the
      context of existing user and business relationships, the details of the enquiring persons are
      processed to the extent necessary to respond to the contact enquiries and any requested
      measures.
    </p>
    <ul>
      <li>
        <b>Types of data processed:</b> Master data (e.g. full name, home address, contact
        information, customer number, etc.); contact data (e.g. postal and email addresses or phone
        numbers); content data (e.g. text or image messages and posts and information relating to
        them, such as details of authorship or time of creation); usage data (e.g. page views and
        dwell time, click paths, intensity and frequency of use, device types and operating systems
        used, interactions with content and functions). Meta, communication and procedural data
        (e.g. IP addresses, timestamps, identification numbers, persons involved).
      </li>
      <li><b>Data subjects:</b> Communication partners.</li>
      <li>
        <b>Purposes of processing:</b> Communication; organisational and administrative procedures;
        feedback (e.g. collecting feedback via online form). Provision of our online offering and
        user-friendliness.
      </li>
      <li>
        <b>Retention and deletion:</b> Deletion in accordance with the information in the section
        "General information on data storage and deletion".
      </li>
      <li>
        <b>Legal bases:</b> Legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR). Performance
        of a contract and pre-contractual enquiries (Art. 6 (1) sentence 1 lit. b) GDPR).
      </li>
    </ul>
    <p><b>Further information on processing operations, procedures and services:</b></p>
    <ul>
      <li>
        <b>Contact form: </b>When contacting us via our contact form, by email or other means of
        communication, we process the personal data transmitted to us in order to respond to and
        handle the respective request. This generally includes details such as name, contact
        information and, where applicable, further information that is communicated to us and is
        necessary for appropriate handling. We use this data exclusively for the stated purpose of
        contact and communication;
        <span
          ><b>Legal bases:</b> Performance of a contract and pre-contractual enquiries (Art. 6 (1)
          sentence 1 lit. b) GDPR), legitimate interests (Art. 6 (1) sentence 1 lit. f) GDPR).</span
        >
      </li>
    </ul>
    <h2 id="13">Plug-ins and embedded functions and content</h2>
    <p>
      We integrate functional and content elements into our online offering that are obtained from
      the servers of their respective providers (hereinafter referred to as "third-party
      providers"). These may be, for example, graphics, videos or city maps (hereinafter uniformly
      referred to as "content").
    </p>
    <p>
      Integration always requires that the third-party providers of this content process the users'
      IP address, as they could not send the content to their browser without the IP address. The IP
      address is therefore necessary for the display of this content or these functions. We strive
      to use only content whose respective providers use the IP address solely to deliver the
      content. Third-party providers may also use so-called pixel tags (invisible graphics, also
      known as "web beacons") for statistical or marketing purposes. The "pixel tags" can be used to
      evaluate information such as visitor traffic on the pages of this website. The pseudonymous
      information may also be stored in cookies on the users' device and may contain, among other
      things, technical information about the browser and operating system, referring websites, time
      of visit and further information on the use of our online offering, and may also be combined
      with such information from other sources.
    </p>
    <p>
      <b>Notes on legal bases:</b> Where we ask users for their consent to the use of third-party
      providers, the legal basis for the data processing is that consent. Otherwise, user data is
      processed on the basis of our legitimate interests (i.e. interest in efficient, economical and
      recipient-friendly services). In this context, we would also like to draw your attention to
      the information on the use of cookies in this privacy policy.
    </p>
    <ul>
      <li>
        <b>Types of data processed:</b> Usage data (e.g. page views and dwell time, click paths,
        intensity and frequency of use, device types and operating systems used, interactions with
        content and functions). Meta, communication and procedural data (e.g. IP addresses,
        timestamps, identification numbers, persons involved).
      </li>
      <li><b>Data subjects:</b> Users (e.g. website visitors, users of online services).</li>
      <li>
        <b>Purposes of processing:</b> Provision of our online offering and user-friendliness.
      </li>
      <li>
        <b>Retention and deletion:</b> Deletion in accordance with the information in the section
        "General information on data storage and deletion". Storage of cookies for up to 2 years
        (Unless otherwise stated, cookies and similar storage methods may be stored on users'
        devices for a period of two years.).
      </li>
      <li>
        <b>Legal bases:</b> Consent (Art. 6 (1) sentence 1 lit. a) GDPR). Legitimate interests (Art.
        6 (1) sentence 1 lit. f) GDPR).
      </li>
    </ul>
    <p><b>Further information on processing operations, procedures and services:</b></p>
    <ul>
      <li>
        <b>Fonts (self-hosted): </b>The fonts used on this website (Bricolage Grotesque and Geist
        Mono) are delivered from our own server. Loading our pages does not trigger any requests to
        third-party servers (e.g. Google) for this purpose, and no data is transmitted to third
        parties.
      </li>
    </ul>
    <h2 id="14">Changes and updates</h2>
    <p>
      We ask you to inform yourself regularly about the content of our privacy policy. We adapt the
      privacy policy as soon as changes to the data processing we carry out make this necessary. We
      will inform you as soon as the changes require an act of cooperation on your part (e.g.
      consent) or other individual notification.
    </p>
    <p>
      Where we provide addresses and contact information of companies and organisations in this
      privacy policy, please note that addresses may change over time and please check the
      information before contacting them.
    </p>
    <p class="text-[12px] text-(--muted)">
      <a
        href="https://datenschutz-generator.de/"
        title="Legal text by Dr. Schwenke - click for further information."
        target="_blank"
        rel="noopener noreferrer nofollow"
        >Created with the free Datenschutz-Generator.de by Dr. Thomas Schwenke</a
      >
    </p>
  </div>
</template>
