
uint __thiscall
FUN_0058bec0(void *this,int *param_1,int *param_2,void *param_3,undefined4 *param_4,
            undefined4 param_5)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  uint *puVar8;
  uint *puVar9;
  int *local_2c;
  uint local_28;
  uint local_24;
  int local_20 [3];
  int local_14;
  int *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = ExceptionList;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fe88;
  local_c = ExceptionList;
  iVar7 = 0;
  ExceptionList = &local_c;
  *param_4 = 0;
  local_2c = (int *)0x0;
  if (((param_1 == (int *)0x0) || (param_2 == (int *)0x0)) || (param_3 == (void *)0x0)) {
    ExceptionList = pvVar1;
    return 0;
  }
  uVar4 = 1;
  iVar2 = FUN_0053e290(*(int *)(*(int *)((int)this + 4) + 0x18) + -1);
  if (iVar2 < *(int *)(*(int *)((int)this + 4) + 0x20)) {
    FUN_0058bd00(local_20,*(int *)((int)this + 4));
    local_4 = 0;
    local_14 = iVar2;
    iVar2 = FUN_0058bbd0(*(void **)((int)this + 4),local_20,param_5);
    uVar4 = 0;
    if (iVar2 != 0) {
      local_2c = local_10;
      uVar4 = (uint)(local_10 != (int *)0x0);
    }
    local_4 = 0xffffffff;
    FUN_0058bd50(local_20);
  }
  if (uVar4 == 0) {
    ExceptionList = local_c;
    return 0;
  }
  if (local_2c == (int *)0x0) {
    ExceptionList = local_c;
    return uVar4;
  }
  iVar6 = 0;
  param_2 = (int *)0x0;
  *(int *)(*(int *)((int)this + 4) + 0x24) = *(int *)(*(int *)((int)this + 4) + 0x24) + 1;
  iVar2 = FUN_00509330(*(void **)(*(int *)((int)this + 4) + 8),3,(int *)&param_2);
  if (iVar2 != 0) {
    if (param_2 != (int *)0x0) {
      iVar7 = (**(code **)(*param_2 + 500))();
    }
    if (iVar2 != 0) {
      param_2 = (int *)0x0;
      iVar2 = FUN_004fd790(param_3,3,(int *)&param_2);
      if ((iVar2 != 0) && (param_2 != (int *)0x0)) {
        iVar6 = (**(code **)(*param_2 + 500))();
      }
    }
  }
  local_28 = 0;
  local_24 = 0;
  if (iVar2 == 0) {
    uVar4 = 0;
  }
  else {
    puVar9 = &local_24;
    puVar8 = &local_28;
    iVar2 = FUN_00520ba0((int)local_2c);
    iVar3 = FUN_00520b90((int)param_1);
    uVar4 = FUN_0055eb00(iVar3,iVar6,iVar2,iVar7,puVar8,puVar9);
  }
  if (uVar4 == 0) {
    ExceptionList = local_c;
    return 0;
  }
  if (local_24 == 0) {
    if (local_28 == 0) {
      ExceptionList = local_c;
      return uVar4;
    }
    if (local_28 == 0) goto LAB_0058c08e;
    *(int *)(*(int *)((int)this + 4) + 0x1c) = *(int *)(*(int *)((int)this + 4) + 0x1c) + -1;
    iVar7 = *param_1;
  }
  else {
    *(int *)(*(int *)((int)this + 4) + 0x20) = *(int *)(*(int *)((int)this + 4) + 0x20) + -1;
    iVar7 = *local_2c;
  }
  uVar4 = (**(code **)(iVar7 + 0xac))(0xe,param_5);
LAB_0058c08e:
  if ((*(int *)(*(int *)((int)this + 4) + 0x20) == 0) ||
     (*(int *)(*(int *)((int)this + 4) + 0x1c) == 0)) {
    uVar5 = 1;
  }
  else {
    uVar5 = 0;
  }
  *param_4 = uVar5;
  ExceptionList = local_c;
  return uVar4;
}

