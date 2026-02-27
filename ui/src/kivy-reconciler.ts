import ReactReconciler from 'react-reconciler';
import type React from 'react';

type HostNode = {
  id: number;
  type: string;
  props: Record<string, unknown>;
};

let nextId = 1;

const kivy = (globalThis as any).Kivy as Window['Kivy'] | undefined;

const KivyHostConfig = {
  supportsMutation: true,
  supportsPersistence: false,
  supportsHydration: false,
  isPrimaryRenderer: true,
  noTimeout: -1,
  now: Date.now,
  getInstanceFromNode: () => null,
  beforeActiveInstanceBlur: () => {},
  afterActiveInstanceBlur: () => {},
  preparePortalMount: () => {},
  scheduleTimeout: () => 0,
  cancelTimeout: () => {},
  getRootHostContext: () => null,
  getChildHostContext: () => null,
  shouldSetTextContent: () => false,
  createTextInstance: () => ({ id: -1, type: 'text', props: {} } as HostNode),
  createInstance: (type: string, props: Record<string, unknown>) => {
    const id = nextId++;
    kivy?.instantiateWidget(id, type, props);
    return { id, type, props } as HostNode;
  },
  appendChild: (parent: HostNode, child: HostNode) => {
    kivy?.bindHierarchy(parent.id, child.id);
  },
  appendChildToContainer: (_container: unknown, child: HostNode) => {
    kivy?.bindHierarchy(0, child.id);
  },
  appendInitialChild: (parent: HostNode, child: HostNode) => {
    kivy?.bindHierarchy(parent.id, child.id);
  },
  insertBefore: (parent: HostNode, child: HostNode) => {
    kivy?.bindHierarchy(parent.id, child.id);
  },
  insertInContainerBefore: (_container: unknown, child: HostNode) => {
    kivy?.bindHierarchy(0, child.id);
  },
  removeChild: (parent: HostNode, child: HostNode) => {
    kivy?.unbindHierarchy(parent.id, child.id);
    kivy?.terminateWidget(child.id);
  },
  removeChildFromContainer: (_container: unknown, child: HostNode) => {
    kivy?.unbindHierarchy(0, child.id);
    kivy?.terminateWidget(child.id);
  },
  finalizeInitialChildren: () => false,
  commitMount: () => {},
  prepareUpdate: () => true,
  commitUpdate: (instance: HostNode, _payload: unknown, _type: string, _oldProps: unknown, newProps: Record<string, unknown>) => {
    instance.props = newProps;
    kivy?.applyPropertyUpdate(instance.id, newProps);
  },
  commitTextUpdate: () => {},
  resetTextContent: () => {},
  clearContainer: () => {},
  detachDeletedInstance: () => {},
  getPublicInstance: (instance: HostNode) => instance,
  prepareForCommit: () => null,
  resetAfterCommit: () => {},
};

const reconciler = (ReactReconciler as any)(KivyHostConfig);

export function renderToKivy(element: React.ReactNode) {
  const container = reconciler.createContainer({}, 0, null, false, null, '', console.error, null);
  reconciler.updateContainer(element, container, null, null);
}
