
void __thiscall FUN_004734d0(void *this,int param_1)

{
  undefined4 uVar1;
  
  *(undefined4 *)((int)this + 0x30) = *(undefined4 *)(param_1 + 100);
  *(undefined4 *)((int)this + 0x34) = *(undefined4 *)(param_1 + 0x68);
  *(undefined4 *)((int)this + 0x38) = *(undefined4 *)(param_1 + 0x60);
  *(undefined4 *)((int)this + 0x3c) = *(undefined4 *)(param_1 + 0x5c);
  uVar1 = FUN_0053b860(param_1);
  *(undefined4 *)((int)this + 0x1c) = uVar1;
  uVar1 = FUN_0053b870(param_1);
  *(undefined4 *)((int)this + 0x20) = uVar1;
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)(param_1 + 0x50);
  if (*(int *)(param_1 + 0x44) == 0) {
    *(undefined4 *)((int)this + 0x28) = 1;
  }
  else {
    *(undefined4 *)((int)this + 0x28) = 2;
  }
  if (*(int *)((int)this + 0x34) <= *(int *)((int)this + 0x30)) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x1000;
  }
  if (*(int *)((int)this + 0x30) <= *(int *)((int)this + 0x34)) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x2000;
  }
  return;
}

