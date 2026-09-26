
undefined4 __thiscall
FUN_00588a90(void *this,int *param_1,void *param_2,int param_3,int param_4,undefined4 *param_5)

{
  void *pvVar1;
  uint uVar2;
  int *piVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  undefined4 uVar8;
  undefined4 *puVar9;
  int **ppiVar10;
  int *local_8;
  int local_4;
  
  pvVar1 = param_2;
  piVar3 = param_1;
  *param_5 = 0;
  local_4 = 0;
  local_8 = (int *)0x0;
  if (param_2 == (void *)0x0) {
    pvVar1 = *(void **)((int)this + 8);
    ppiVar10 = &local_8;
    iVar4 = (**(code **)(*param_1 + 0x1bc))();
    iVar4 = FUN_00509330(pvVar1,iVar4,(int *)ppiVar10);
  }
  else {
    ppiVar10 = &local_8;
    iVar4 = (**(code **)(*param_1 + 0x1bc))();
    iVar4 = FUN_004fd790(pvVar1,iVar4,(int *)ppiVar10);
  }
  if (local_8 != (int *)0x0) {
    local_4 = (**(code **)(*local_8 + 0x1e0))();
  }
  param_1 = (int *)0x0;
  uVar2 = piVar3[9];
  ppiVar10 = &param_1;
  uVar5 = (uint)(param_2 != (void *)0x0);
  iVar7 = param_4;
  puVar9 = param_5;
  iVar6 = (**(code **)(*piVar3 + 0x1c4))();
  iVar7 = FUN_0055e470(param_3,local_4,uVar2 >> 6 & 3,iVar6,iVar7,uVar5,puVar9,(int *)ppiVar10);
  if ((iVar7 == 0) || (iVar4 == 0)) {
    uVar8 = 0;
  }
  else {
    uVar8 = 1;
  }
  FUN_005897b0();
  return uVar8;
}

