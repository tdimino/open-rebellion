
uint FUN_00555460(int *param_1,int *param_2,uint *param_3)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  uint *puVar4;
  uint *puVar5;
  void *local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649438;
  local_c = ExceptionList;
  local_14 = (void *)0x0;
  local_10 = (void *)0x0;
  ExceptionList = &local_c;
  *param_3 = 1;
  uVar2 = FUN_00555540(param_1,(int *)&local_14);
  uVar3 = 0;
  if (uVar2 != 0) {
    uVar3 = FUN_00555540(param_2,(int *)&local_10);
    if (((uVar3 != 0) && (local_14 != (void *)0x0)) && (local_10 != (void *)0x0)) {
      puVar4 = FUN_004025b0(local_10,(uint *)&param_1);
      local_4 = 0;
      puVar5 = FUN_004025b0(local_14,(uint *)&param_3);
      *puVar1 = (uint)(*puVar5 == *puVar4);
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

