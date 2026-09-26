
void __thiscall FUN_00473090(void *this,int param_1)

{
  bool bVar1;
  undefined4 uVar2;
  
  *(undefined4 *)((int)this + 0x30) = *(undefined4 *)(param_1 + 0x5c);
  uVar2 = FUN_0053b860(param_1);
  *(undefined4 *)((int)this + 0x1c) = uVar2;
  uVar2 = FUN_0053b870(param_1);
  *(undefined4 *)((int)this + 0x20) = uVar2;
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)(param_1 + 0x50);
  bVar1 = false;
  if (*(int *)(param_1 + 0x44) == 0) {
    *(undefined4 *)((int)this + 0x28) = 1;
  }
  else if (*(int *)(param_1 + 0x40) == 0) {
    *(undefined4 *)((int)this + 0x28) = 2;
  }
  else {
    *(undefined4 *)((int)this + 0x28) = 0;
  }
  if ((0x27 < *(uint *)(param_1 + 0x2c)) && (*(uint *)(param_1 + 0x2c) < 0x29)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x4000;
  }
  if ((*(uint *)(param_1 + 0x2c) < 0x29) || (0x29 < *(uint *)(param_1 + 0x2c))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x8000;
  }
  if ((*(uint *)(param_1 + 0x2c) < 0x2a) || (0x2a < *(uint *)(param_1 + 0x2c))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x10000;
  }
  return;
}

