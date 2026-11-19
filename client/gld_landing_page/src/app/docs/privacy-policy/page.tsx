import DocsPage from '@/components/docs/DocsPage'
import { docsMetadata } from '@/docs/metadata'

export const metadata = docsMetadata('privacy-policy')

// Rewritten in November 2026. The earlier text, from docs.gold-dao.org, only
// covered the newsletter sign-up of the GLDT app, which moved to ORIGYN with
// that app. This text describes what the website and the Gold DAO app do with
// data. When one of them starts to store or send something new (analytics, a
// form, another outside host), this page has to change with it and get a new
// date.
export default function Page () {
  return (
    <DocsPage
      slug='privacy-policy'
      lead={
        <>
          <strong>Last updated:</strong> 19 November 2026
        </>
      }
    >
      <p>
        Gold DAO (&quot;we,&quot; &quot;our,&quot; &quot;us&quot;) values your
        privacy. This Privacy Policy explains which data is processed when you
        use the Gold DAO website (www.gold-dao.org) and the Gold DAO app
        (app.gold-dao.org), together the &quot;Services&quot;.
      </p>

      <h2>1. What we do not collect</h2>
      <p>
        The Services do not ask for your name, your email address or any other
        personal details. They have no user accounts and use no analytics, no
        advertising and no tracking cookies.
      </p>
      <p>
        We do <strong>not</strong> sell, rent, or trade personal data.
      </p>

      <h2>2. Data stored in your browser</h2>
      <ul>
        <li>
          The website sets one technical cookie, <code>ic_env</code>. It comes
          from the canister that hosts the website and holds the public key of
          the Internet Computer network. It contains no personal data.
        </li>
        <li>
          The app keeps your display settings and your sign-in session in the
          storage of your browser. Clearing the site data in your browser
          removes both.
        </li>
      </ul>

      <h2>3. Data on the blockchain</h2>
      <p>
        The app works with smart contracts (canisters) on the Internet
        Computer. When you sign in, the app receives a principal, a
        pseudonymous identifier that your wallet or identity provider creates.
      </p>
      <p>
        Transactions you make through the app, such as claiming rewards or
        sending tokens, are recorded on public ledgers together with your
        principal. This data is public and permanent. Nobody, including Gold
        DAO, can change or delete it.
      </p>

      <h2>4. Third parties</h2>
      <p>
        Using the Services involves third parties that process technical data
        such as your IP address:
      </p>
      <ul>
        <li>
          <strong>Internet Computer</strong>: the website and the app are
          served by canisters through the gateways of the network. The gateways
          process your IP address to deliver the pages.
        </li>
        <li>
          <strong>Adobe Fonts</strong>: the typeface is loaded from servers of
          Adobe, which receives your IP address when a page loads.
        </li>
        <li>
          <strong>Data API</strong>: the website loads the treasury figures
          from api.gold-dao.org, and the app loads the gold price from there.
        </li>
        <li>
          <strong>Sign-in</strong>: the app lets you sign in with Internet
          Identity, NFID Wallet or OISY. These services have their own privacy
          policies.
        </li>
      </ul>
      <p>
        The Services link to other websites, such as the NNS dapp, the ICP
        dashboard and exchanges. Their operators are responsible for what
        happens there.
      </p>

      <h2>5. Your rights</h2>
      <p>
        Depending on your jurisdiction, you may have the right to access,
        correct or delete personal data that is held about you, and to lodge a
        complaint with your local data protection authority.
      </p>
      <p>
        The Services keep no user profiles, so Gold DAO has no personal data to
        hand over, correct or delete. Data on public ledgers cannot be deleted.
        For data that the third parties named above process, please turn to
        them.
      </p>

      <h2>6. International transfers</h2>
      <p>
        The Internet Computer and the third parties named above operate in many
        countries. Data may therefore be processed in jurisdictions that have
        different data protection laws than your own.
      </p>

      <h2>7. Children&apos;s privacy</h2>
      <p>
        Our Services are not directed at individuals under the age of 18. We do
        not knowingly collect personal data from minors.
      </p>

      <h2>8. Changes to this policy</h2>
      <p>
        We may update this Privacy Policy from time to time. The &quot;Last
        updated&quot; date will always indicate the latest version. We
        encourage you to review this page periodically.
      </p>
    </DocsPage>
  )
}
