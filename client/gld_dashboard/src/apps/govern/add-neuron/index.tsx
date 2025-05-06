import clsx from "clsx";
import { ReactNode } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { Button, ExternalLink } from "@components/index";
import { AddNeuronStateReducerAtom } from "./atoms";
import Address from "@components/strings/Address";

const Card = ({ step, children }: { step: number; children: ReactNode }) => {
  return (
    <div className="flex items-center gap-4 bg-surface-secondary border border-border rounded-md p-4">
      <div className="flex items-center justify-center h-10 w-10 rounded-full bg-surface-primary text-primary border border-primary shrink-0">
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

const AddNeuron = () => {
  const { principalId } = useAuth();
  const [, dispatch] = useAtom(AddNeuronStateReducerAtom);

  return (
    <>
      <div className="text-center mt-4 mb-6 text-4xl xl:text-5xl">
        Add <span className="font-semibold text-primary">Neuron</span>
      </div>
      <div className="my-8 grid grid-cols-1 gap-4">
        <Card step={1}>
          <Card.Title>
            Copy you principal from the Gold DAO dashboard
          </Card.Title>
          <Card.Text>
            <Address value={principalId} size="lg" />
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
            Refresh your Gold DAO dashboard and you will now see your neuron
            linked.
          </Card.Text>
        </Card>
      </div>
      <Button
        className={clsx(
          "px-4 py-3 rounded-md w-full",
          "bg-secondary text-white"
        )}
        onClick={() => dispatch({ type: "RESET" })}
      >
        Got it!
      </Button>
    </>
  );
};

export default AddNeuron;
