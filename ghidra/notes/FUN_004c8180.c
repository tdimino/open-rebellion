
bool __thiscall FUN_004c8180(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  uint uVar4;
  
  FUN_004f26d0((void *)((int)this + 0x28),(uint *)(param_1 + 0x2c));
  uVar4 = *(uint *)(param_1 + 0x2c) >> 0x18;
  if ((uVar4 < 0x90) || (0x97 < uVar4)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  *(undefined4 *)((int)this + 0x18) = *(undefined4 *)(param_1 + 0x20);
  iVar1 = *(int *)(param_1 + 0x20);
  iVar2 = *(int *)(param_1 + 0x24);
  *(int *)((int)this + 0x1c) = iVar2;
  if ((iVar2 == 1) || (iVar2 == 2)) {
    *(undefined4 *)((int)this + 0x20) = 1;
  }
  else if (iVar2 == 3) {
    *(undefined4 *)((int)this + 0x20) = 2;
  }
  else {
    *(undefined4 *)((int)this + 0x20) = 0;
  }
  FUN_004fcd90((void *)((int)this + 0x24),(undefined4 *)(param_1 + 0x28));
  return iVar1 != 0 && bVar3;
}

