
void __thiscall FUN_004736c0(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  
  iVar1 = *(int *)((int)param_1 + 200);
  *(int *)((int)this + 0x4c) = iVar1;
  *(int *)((int)this + 0x38) = (iVar1 * 0x19) / 10;
  *(undefined4 *)((int)this + 0x3c) = *(undefined4 *)((int)param_1 + 0x68);
  *(undefined4 *)((int)this + 0x40) = *(undefined4 *)((int)param_1 + 0xcc);
  *(int *)((int)this + 0x48) = *(int *)((int)param_1 + 0xc0) * *(int *)((int)this + 0x4c);
  iVar1 = FUN_0053bb80(param_1,0,1);
  iVar2 = FUN_0053bb80(param_1,0,2);
  iVar1 = iVar1 * 100 + (iVar2 * 100) / 6;
  if (0 < iVar1) {
    iVar2 = FUN_0053bb80(param_1,0,2);
    iVar3 = FUN_0053bb80(param_1,0,1);
    *(int *)((int)this + 0x30) = (iVar1 * *(int *)((int)this + 0x4c)) / (iVar2 + iVar3);
  }
  iVar1 = FUN_0053bb80(param_1,0,2);
  if (0 < iVar1) {
    iVar2 = FUN_0053bb80(param_1,0,2);
    iVar2 = (iVar1 * *(int *)((int)this + 0x3c) * *(int *)((int)this + 0x4c) * 10) / iVar2;
    *(int *)((int)this + 0x34) = iVar2;
    iVar1 = *(int *)((int)param_1 + 0x60) * iVar2;
    *(int *)((int)this + 0x34) = ((int)(iVar1 + (iVar1 >> 0x1f & 7U)) >> 3) + iVar2;
  }
  uVar4 = FUN_0053b860((int)param_1);
  *(undefined4 *)((int)this + 0x1c) = uVar4;
  uVar4 = FUN_0053b870((int)param_1);
  *(undefined4 *)((int)this + 0x20) = uVar4;
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)((int)param_1 + 0x50);
  *(undefined4 *)((int)this + 0x44) = *(undefined4 *)((int)param_1 + 0x5c);
  *(uint *)((int)this + 0x18) = *(uint *)((int)this + 0x18) | 0x200000;
  if (*(int *)((int)param_1 + 0x44) != 0) {
    *(undefined4 *)((int)this + 0x28) = 2;
    return;
  }
  *(undefined4 *)((int)this + 0x28) = 1;
  return;
}

