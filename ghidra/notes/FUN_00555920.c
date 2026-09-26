
uint FUN_00555920(uint *param_1,uint *param_2,void *param_3)

{
  bool bVar1;
  uint *puVar2;
  void *this;
  int *piVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649508;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar2 = param_1;
  piVar3 = (int *)FUN_00504dc0(param_1);
  uVar5 = 0;
  if ((piVar3 != (int *)0x0) == 0) goto LAB_00555ad6;
  iVar4 = (**(code **)(*piVar3 + 0x38))();
  uVar6 = (uint)(piVar3 != (int *)0x0);
  if (iVar4 != 0) {
    param_1 = (uint *)0x0;
    uVar5 = FUN_00555410(puVar2,param_2,(uint *)&param_1);
    if (uVar5 == 0) goto LAB_00555ad6;
    if ((param_1 == (uint *)0x0) && (iVar4 = (**(code **)(*piVar3 + 0x34))(1), iVar4 == 0)) {
      local_14 = 1;
      local_10 = 0x18;
      local_4 = 1;
      FUN_00520580(param_3,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    this = param_3;
    if (uVar5 == 0) goto LAB_00555ad6;
    uVar6 = uVar5;
    if ((param_1 == (uint *)0x0) && (*(int *)((int)param_3 + 4) == -1)) {
      local_14 = 0x10;
      local_10 = 0x14;
      local_4 = 2;
      uVar5 = (**(code **)(*piVar3 + 4))();
      if ((uVar5 < local_14) || (local_10 <= uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        iVar4 = FUN_00504dc0(param_2);
        uVar6 = (uint)(iVar4 != 0);
        uVar5 = 0;
        if (uVar6 == 0) goto LAB_00555ad6;
        if (((*(uint *)(iVar4 + 0x24) ^ piVar3[9]) & 0xc0) != 0) {
          local_14 = 1;
          local_10 = 0x28;
          local_4 = 3;
          FUN_00520580(this,&local_14);
          local_4 = 0xffffffff;
          FUN_00619730();
        }
      }
    }
  }
  uVar5 = 0;
  if (uVar6 != 0) {
    ExceptionList = pvStack_c;
    return uVar6;
  }
LAB_00555ad6:
  local_14 = 1;
  local_10 = 1;
  local_4 = 4;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar5;
}

