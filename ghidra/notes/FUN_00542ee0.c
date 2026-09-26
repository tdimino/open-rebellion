
uint FUN_00542ee0(int *param_1,void *param_2,uint *param_3,uint *param_4)

{
  bool bVar1;
  uint *puVar2;
  int iVar3;
  uint *puVar4;
  uint uVar5;
  int local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar4 = param_4;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006471b0;
  local_c = ExceptionList;
  uVar5 = 1;
  ExceptionList = &local_c;
  FUN_004ece80(param_4);
  puVar2 = FUN_004025b0(param_2,(uint *)&param_2);
  local_14 = 0x50;
  local_10 = 0x80;
  local_4 = 0;
  if ((*puVar2 >> 0x18 < 0x50) || (0x7f < *puVar2 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (!bVar1) goto LAB_0054300c;
  iVar3 = thunk_FUN_00506e60();
  uVar5 = (uint)(iVar3 != 0);
  local_14 = 1;
  local_10 = 1;
  local_4 = 1;
  if (uVar5 != 0) {
    if (param_3 != (uint *)0x0) {
      uVar5 = FUN_00554170(param_1,&local_14);
      if (uVar5 == 0) goto LAB_00542ffb;
      if (local_10 == -1) {
        uVar5 = FUN_004f6b70(param_1,puVar4);
      }
    }
    if ((uVar5 != 0) && (local_10 != -1)) {
      uVar5 = FUN_005543c0(param_1,(uint *)(param_1 + 0x1c),&local_14);
      puVar4 = param_4;
      if ((uVar5 != 0) && (local_10 == -1)) {
        FUN_004f26d0(param_4,param_1 + 0x1c);
        puVar4 = param_4;
      }
    }
  }
LAB_00542ffb:
  local_4 = 0xffffffff;
  FUN_00619730();
LAB_0054300c:
  iVar3 = FUN_004ece60(puVar4);
  if (iVar3 == 0) {
    uVar5 = FUN_00543040(param_1,param_3,puVar4);
  }
  ExceptionList = local_c;
  return uVar5;
}

