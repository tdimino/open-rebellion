
uint __thiscall FUN_005849c0(void *this,int *param_1)

{
  uint uVar1;
  int iVar2;
  uint uVar3;
  int local_74;
  int local_70;
  int aiStack_6c [4];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f398;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(&local_70,1);
  local_74 = 0;
  local_4 = 0;
  uVar3 = 0;
  if (*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0) {
    FUN_00584e60(param_1,&local_70,&local_74);
    iVar2 = *(int *)((int)aiStack_6c + *(int *)(local_70 + 4));
    uVar3 = *(uint *)(iVar2 + 0x1c);
    uVar1 = *(uint *)(iVar2 + 0x18);
    if (uVar3 < uVar1) {
      iVar2 = 0;
    }
    else {
      iVar2 = uVar3 - uVar1;
    }
    uVar3 = 0;
    if (*(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1) == 0 && iVar2 != 0) {
      uVar3 = (**(code **)(*(int *)this + 0x14))(param_1,&local_74);
    }
  }
  while (local_74 == 0) {
    FUN_00584e60(param_1,&local_70,&local_74);
  }
  local_4 = 0xffffffff;
  FUN_00615ea0((int)auStack_5c);
  FUN_00615440(auStack_5c);
  ExceptionList = pvStack_c;
  return uVar3;
}

