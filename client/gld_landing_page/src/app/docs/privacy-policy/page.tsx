import DocsPage from '@/components/docs/DocsPage'
import { docsMetadata } from '@/docs/metadata'

export const metadata = docsMetadata('privacy-policy')

// The wording is the one published on docs.gold-dao.org, without its last
// section, "10. Contact", and without the sentence in section 6 that pointed
// to it. Gold DAO is a DAO and names no contact.
export default function Page () {
  return (
    <DocsPage
      slug='privacy-policy'
      lead={
        <>
          <strong>Last Updated:</strong> 29 September 2025
        </>
      }
    >
      <p>
        Gold DAO (&quot;we,&quot; &quot;our,&quot; &quot;us&quot;) values your
        privacy. This Privacy Policy explains how we collect, use, and protect
        your personal data when you provide your email address to subscribe to
        our newsletter or receive updates from Gold DAO (the
        &quot;Services&quot;).
      </p>
      <p>
        By submitting your email address, you agree to the terms of this
        Privacy Policy.
      </p>

      <h2>1. Information We Collect</h2>
      <p>We collect the following personal information from you:</p>
      <ul>
        <li>
          <strong>Email address</strong>: when you voluntarily subscribe to our
          newsletter, updates, or announcements.
        </li>
      </ul>
      <p>
        We do <strong>not</strong> collect any additional personal data through
        the newsletter form.
      </p>

      <h2>2. How We Use Your Information</h2>
      <p>We use your email address for the following purposes:</p>
      <ul>
        <li>
          To send periodic newsletters, updates, or announcements related to
          Gold DAO.
        </li>
        <li>
          To inform you of new products, governance developments, events, or
          ecosystem updates.
        </li>
        <li>
          To improve our communications and better understand community
          engagement.
        </li>
      </ul>
      <p>
        We will <strong>not</strong> sell, rent, or trade your email address
        with third parties.
      </p>

      <h2>3. Legal Basis for Processing</h2>
      <p>
        We process your email address based on <strong>your consent</strong>.
        You may withdraw your consent at any time by unsubscribing using the
        link provided in every email.
      </p>

      <h2>4. Data Storage &amp; Security</h2>
      <ul>
        <li>
          Your email address is stored securely using reputable third-party
          newsletter or mailing list providers.
        </li>
        <li>
          We implement appropriate technical and organizational measures to
          protect your data against unauthorized access, alteration,
          disclosure, or destruction.
        </li>
      </ul>

      <h2>5. Data Retention</h2>
      <p>
        We will retain your email address for as long as you remain subscribed
        to our newsletter. Once you unsubscribe, your email will be deleted or
        anonymized within a reasonable period.
      </p>

      <h2>6. Your Rights</h2>
      <p>Depending on your jurisdiction, you may have the right to:</p>
      <ul>
        <li>Access the personal data we hold about you.</li>
        <li>Request correction or deletion of your data.</li>
        <li>Withdraw your consent to receive newsletters.</li>
        <li>Lodge a complaint with your local data protection authority.</li>
      </ul>

      <h2>7. International Transfers</h2>
      <p>
        Gold DAO operates globally. By providing your email address, you
        consent to the transfer and processing of your data in jurisdictions
        that may have different data protection laws than your own.
      </p>

      <h2>8. Children&apos;s Privacy</h2>
      <p>
        Our Services are not directed at individuals under the age of 18. We do
        not knowingly collect personal data from minors.
      </p>

      <h2>9. Changes to This Policy</h2>
      <p>
        We may update this Privacy Policy from time to time. The &quot;Last
        Updated&quot; date will always indicate the latest version. We
        encourage you to review this page periodically.
      </p>

    </DocsPage>
  )
}
