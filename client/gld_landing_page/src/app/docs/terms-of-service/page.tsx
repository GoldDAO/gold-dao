import DocsPage from '@/components/docs/DocsPage'
import { docsMetadata } from '@/docs/metadata'

export const metadata = docsMetadata('terms-of-service')

// The wording is the one published on docs.gold-dao.org, without its last
// section, "14. Contact". Gold DAO is a DAO and names no contact.
export default function Page () {
  return (
    <DocsPage
      slug='terms-of-service'
      lead={
        <>
          Welcome to{' '}
          <strong>Gold DAO (&quot;we,&quot; &quot;our,&quot; &quot;us&quot;)</strong>
          . By accessing or using our website, smart contracts, tokens, or
          related services (collectively, the &quot;Services&quot;), you
          (&quot;you,&quot; &quot;your,&quot; &quot;User&quot;) agree to be bound
          by these Terms of Service (&quot;Terms&quot;). Please read them
          carefully before using the Services.
        </>
      }
    >
      <p>If you do not agree to these Terms, do not use our Services.</p>

      <h2>1. Eligibility</h2>
      <ul>
        <li>
          You must be at least 18 years old or the legal age of majority in
          your jurisdiction to use the Services.
        </li>
        <li>
          You represent and warrant that you are not a citizen or resident of
          any jurisdiction in which the use of our Services is prohibited by
          law.
        </li>
        <li>
          You are solely responsible for complying with all applicable laws and
          regulations.
        </li>
      </ul>

      <h2>2. Services Provided</h2>
      <ul>
        <li>
          Gold DAO facilitates decentralized access to digital assets,
          including the <strong>GOLDAO token</strong> and{' '}
          <strong>GLDT token</strong> and related smart contract protocols.
        </li>
        <li>
          The Services may involve staking, reward distribution, governance
          participation, and representations of real-world assets (RWA), such
          as gold-backed tokens.
        </li>
        <li>
          We do not guarantee uninterrupted access, error-free operation, or
          permanent availability of the Services.
        </li>
      </ul>

      <h2>3. No Financial Advice</h2>
      <ul>
        <li>
          The Services are provided for informational and technological
          purposes only.
        </li>
        <li>
          <strong>
            Gold DAO does not provide investment, financial, tax, or legal
            advice.
          </strong>
        </li>
        <li>
          You acknowledge that cryptocurrency and tokenized assets are
          inherently risky and may lose value. You are solely responsible for
          your decisions.
        </li>
      </ul>

      <h2>4. User Responsibilities</h2>
      <p>By using the Services, you agree to:</p>
      <ul>
        <li>
          Maintain control of your private keys, wallets, and access
          credentials.
        </li>
        <li>Use the Services only in compliance with applicable laws.</li>
        <li>
          Not attempt to interfere with or exploit the Services for unlawful or
          fraudulent purposes.
        </li>
        <li>
          Bear full responsibility for any transactions initiated through your
          wallet.
        </li>
      </ul>

      <h2>5. Token Usage</h2>
      <ul>
        <li>
          The <strong>GLDT token</strong> is a blockchain-based digital asset
          and does not represent legal ownership of physical gold or any equity
          in Gold DAO.
        </li>
        <li>
          While GLDT may be backed or collateralized by gold reserves,{' '}
          <strong>
            Gold DAO makes no guarantee of redemption, convertibility, or
            liquidity
          </strong>
          .
        </li>
        <li>
          The value of GLDT and other tokens may fluctuate significantly,
          including a total loss of value.
        </li>
      </ul>

      <h2>6. Integrated Risk Disclosure</h2>
      <p>
        By using the Services, you acknowledge and accept the following risks:
      </p>
      <ol>
        <li>
          <strong>Market Risk</strong>: Digital assets are highly volatile.
          Prices can fluctuate widely within short periods and may result in
          substantial or total losses.
        </li>
        <li>
          <strong>Smart Contract Risk</strong>: Smart contracts may contain
          vulnerabilities, coding errors, or exploits that could lead to loss
          of funds.
        </li>
        <li>
          <strong>Custody &amp; Wallet Risk</strong>: You are solely responsible
          for safeguarding your wallet and private keys. Loss of access
          credentials will result in permanent loss of assets.
        </li>
        <li>
          <strong>Regulatory Risk</strong>: Digital assets and RWAs may be
          subject to evolving laws and regulations. Access to the Services may
          be restricted, suspended, or prohibited in certain jurisdictions.
        </li>
        <li>
          <strong>Liquidity Risk</strong>: GLDT and other tokens may not have
          continuous market liquidity, making it difficult or impossible to
          buy, sell, or exchange assets at a given price.
        </li>
        <li>
          <strong>Technology Risk</strong>: System outages, cyberattacks,
          malicious actors, or blockchain network failures may disrupt the
          Services or cause asset losses.
        </li>
        <li>
          <strong>Counterparty &amp; Collateral Risk</strong>: While GLDT may be
          linked to audited gold reserves, Gold DAO cannot guarantee against
          risks related to custodians, vault operators, or other third parties.
        </li>
        <li>
          <strong>Governance Risk</strong>: Participation in DAO governance
          carries uncertainty, as community proposals may materially affect the
          Services, token economics, or governance rights.
        </li>
      </ol>
      <p>
        You expressly acknowledge that you understand and assume these risks
        when using the Services.
      </p>

      <h2>7. Limitation of Liability</h2>
      <p>To the fullest extent permitted by law:</p>
      <ul>
        <li>
          Gold DAO, its contributors, developers, and affiliates shall not be
          liable for any indirect, incidental, consequential, or punitive
          damages.
        </li>
        <li>
          Our aggregate liability for any claim related to the Services shall
          not exceed the amount you paid (if any) to use the Services.
        </li>
      </ul>

      <h2>8. Indemnification</h2>
      <p>
        You agree to indemnify, defend, and hold harmless Gold DAO, its
        contributors, and affiliates from any claims, damages, or expenses
        arising from your use of the Services or violation of these Terms.
      </p>

      <h2>9. Intellectual Property</h2>
      <ul>
        <li>
          All content provided through the Services, including branding, logos,
          text, and code, is owned by or licensed to Gold DAO.
        </li>
        <li>
          You may not copy, reproduce, or exploit the Services except as
          permitted under open-source licenses (if applicable).
        </li>
      </ul>

      <h2>10. Third-Party Services</h2>
      <ul>
        <li>
          The Services may integrate or interact with third-party protocols,
          wallets, or exchanges.
        </li>
        <li>
          Gold DAO is not responsible for any third-party services, and your
          use of them is at your own risk.
        </li>
      </ul>

      <h2>11. Termination</h2>
      <ul>
        <li>
          We may suspend or terminate your access to the Services at any time
          without prior notice, particularly if you breach these Terms.
        </li>
        <li>
          Provisions relating to disclaimers, limitations of liability, and
          indemnification shall survive termination.
        </li>
      </ul>

      <h2>12. Governing Law</h2>
      <ul>
        <li>
          These Terms shall be governed by and construed under the laws of{' '}
          <strong>Switzerland</strong>, without regard to conflict-of-law
          principles.
        </li>
        <li>
          Any disputes shall be subject to the exclusive jurisdiction of the
          competent courts in <strong>Neuchâtel, Switzerland</strong>, unless
          otherwise mandated by law.
        </li>
      </ul>

      <h2>13. Amendments</h2>
      <p>
        We may modify or update these Terms from time to time. Continued use of
        the Services after changes are posted constitutes your acceptance of
        the revised Terms.
      </p>

    </DocsPage>
  )
}
