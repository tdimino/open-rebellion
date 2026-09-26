
void __thiscall FUN_005af910(void *this,int *param_1)

{
  void *pvVar1;
  int *piVar2;
  int iVar3;
  uint uVar4;
  
  iVar3 = param_1[0x5c];
  if (iVar3 == 9) {
    return;
  }
  if (iVar3 == 0xb) {
    return;
  }
  if (iVar3 == 2) {
    uVar4 = param_1[0x3b];
    pvVar1 = (void *)((int)this + 100);
  }
  else {
    if (iVar3 != 1) goto LAB_005af956;
    uVar4 = param_1[0x3b];
    pvVar1 = (void *)((int)this + 0x58);
  }
  FUN_005c60d0(pvVar1,uVar4);
LAB_005af956:
  uVar4 = param_1[0x5d];
  param_1[0x5c] = 9;
  pvVar1 = (void *)FUN_00596bd0();
  piVar2 = (int *)FUN_005a6480(pvVar1,uVar4);
  if ((piVar2 != (int *)0x0) && (FUN_005caed0(piVar2,param_1[0x3b]), piVar2[0x1b] == 0)) {
    FUN_005f54a0((void *)(*(int *)((int)this + piVar2[0x14] * 4 + 0x24c) + 0x70),piVar2[6]);
  }
  iVar3 = (**(code **)(*param_1 + 0x20))();
  if (iVar3 != 0) {
    FUN_005c60d0((void *)(*(int *)((int)this + param_1[0x5a] * 4 + 0x24c) + 100),param_1[0x3b]);
    return;
  }
  FUN_005c60d0((void *)(*(int *)((int)this + param_1[0x5a] * 4 + 0x24c) + 0x58),param_1[0x3b]);
  return;
}

