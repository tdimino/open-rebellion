
void __thiscall FUN_00584470(void *this,int *param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  bool bVar4;
  
  bVar4 = param_1 != (int *)0x0;
  if (bVar4) {
    iVar1 = *(int *)((int)this + 0x20);
    iVar2 = (**(code **)(*param_1 + 4))();
    bVar4 = iVar1 == iVar2;
  }
  uVar3 = (uint)param_1[9] >> 4 & 3;
  if (2 < uVar3) {
    bVar4 = false;
  }
  if ((bVar4) && (param_1 == *(int **)((int)this + uVar3 * 4 + 0x24))) {
    *(undefined4 *)((int)this + uVar3 * 4 + 0x24) = 0;
    param_1[8] = 0;
  }
  return;
}

