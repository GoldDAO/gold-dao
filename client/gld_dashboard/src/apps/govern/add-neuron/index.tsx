import { ReactNode } from "react";
import { useAuth } from "@auth/index";
import { ExternalLink } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import Address from "@components/strings/Address";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const Card = ({ step, children }: { step: number; children: ReactNode }) => {
  return (
    <div className="flex items-center gap-4 bg-surface-secondary border border-border rounded-md p-4">
      <div className="flex items-center justify-center h-10 w-10 rounded-full bg-surface-primary text-gold border border-gold shrink-0">
        {step}
      </div>
      <div>{children}</div>
    </div>
  );
};

Card.Title = ({ children }: { children: ReactNode }) => {
  return <div className="font-semibold">{children}</div>;
};

Card.Text = ({ children }: { children: ReactNode }) => {
  return <div className="text-content/60">{children}</div>;
};

const AddNeuron = ({
  open,
  handleClose,
}: {
  open: boolean;
  handleClose: () => void;
}) => {
  const { principalId } = useAuth();

  return (
    <Dialog open={open} handleOnClose={handleClose}>
      <div className="mt-4 mb-6 px-4 xl:px-8">
        <div className="text-center mb-2 text-4xl xl:text-5xl">
          Add <span className="font-semibold text-gold">Neuron</span>
        </div>
        <div className="flex justify-center">
          <div className="text-content/60 text-center">
            In order to add a neuron to the Gold DAO dApp, you need to own a
            GOLDAO neuron. Follow the steps described in{" "}
            <ExternalLink
              href="https://docs.gold-dao.org/how-to/stake-the-goldao-tokens"
              className="text-copper"
            >
              this guide
            </ExternalLink>{" "}
            in order to obtain GOLDAO tokens and stake them in a GOLDAO neuron.
          </div>
        </div>

        <div className="my-8 grid grid-cols-1 gap-4">
          <Card step={1}>
            <Card.Title>Copy you principal from the Gold DAO dApp</Card.Title>
            <Card.Text>
              <Address size="auto">{principalId}</Address>
            </Card.Text>
          </Card>
          <Card step={2}>
            <Card.Title>
              Login to the{" "}
              <span>
                <ExternalLink href="https://nns.ic0.app/neurons/?u=tw2vt-hqaaa-aaaaq-aab6a-cai">
                  NNS dApp
                </ExternalLink>
              </span>
            </Card.Title>
            <Card.Text>to manage your Gold DAO neurons</Card.Text>
          </Card>
          <Card step={3}>
            <Card.Title>Add your principal as hotkey to each neuron</Card.Title>
            <Card.Text>
              Link your neurons by adding your previously copied principal as a
              "hotkey" to each of your neurons
            </Card.Text>
          </Card>
          <Card step={4}>
            <Card.Title>It's done!</Card.Title>
            <Card.Text>
              Refresh your Gold DAO dApp and you will now see your neuron
              linked.
            </Card.Text>
          </Card>
        </div>
        <BtnPrimary className="w-full" onClick={handleClose}>
          Got it!
        </BtnPrimary>
      </div>
    </Dialog>
  );
};

export default AddNeuron;
