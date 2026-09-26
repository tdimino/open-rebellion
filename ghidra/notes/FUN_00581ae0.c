
int FUN_00581ae0(undefined4 param_1,int param_2,void *param_3)

{
  bool bVar1;
  void *this;
  uint *puVar2;
  int iVar3;
  undefined4 uVar4;
  undefined3 extraout_var;
  int iVar5;
  void *this_00;
  undefined4 uVar6;
  undefined1 local_68 [4];
  int local_64 [4];
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_3;
  puStack_8 = &LAB_0064ed60;
  pvStack_c = ExceptionList;
  local_64[0] = 1;
  local_64[1] = 0xffffffff;
  iVar5 = 0;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,local_64);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_004f4340(local_64 + 2);
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  local_64[0] = FUN_0053f150(param_1,param_2,local_64 + 2,this_00);
  param_2 = 0;
  if ((local_64[0] != 0) && (*(int *)((int)this_00 + 4) == -1)) {
    this = (void *)thunk_FUN_005f5060((int)(local_64 + 2));
    for (; (iVar5 == 0 && (this != (void *)0x0)); this = *(void **)((int)this + 0x10)) {
      puVar2 = FUN_00403040(this,&param_1);
      local_54 = 8;
      local_50 = 0x10;
      local_4._0_1_ = 2;
      if ((*puVar2 >> 0x18 < 8) || (0xf < *puVar2 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 1;
      FUN_00619730();
      if (bVar1) {
        puVar2 = FUN_00403040(this,local_68);
        local_4._0_1_ = 3;
        iVar3 = FUN_00506480(puVar2);
        local_4._0_1_ = 1;
        FUN_00619730();
        if (((iVar3 != 0) && ((*(byte *)(iVar3 + 0x50) & 1) != 0)) &&
           ((*(byte *)(iVar3 + 0x58) & 0x20) != 0)) {
          param_2 = 1;
          FUN_00502db0(local_4c,iVar3,1);
          local_4 = CONCAT31(local_4._1_3_,4);
          FUN_00513120((int)local_4c);
          while ((iVar5 == 0 && (local_30 != 0))) {
            uVar6 = 1;
            uVar4 = FUN_0052bed0((int)local_4c);
            FUN_00504c40(local_2c,uVar4,uVar6);
            local_4._0_1_ = 5;
            bVar1 = FUN_005131b0((int)local_2c);
            iVar5 = CONCAT31(extraout_var,bVar1);
            local_4 = CONCAT31(local_4._1_3_,4);
            FUN_00504d40(local_2c);
            FUN_005130d0((int)local_4c);
          }
          local_4._0_1_ = 1;
          FUN_00502eb0(local_4c);
        }
      }
      this_00 = param_3;
    }
  }
  iVar3 = local_64[0];
  if (local_64[0] != 0) {
    if ((*(int *)((int)this_00 + 4) == -1) && (param_2 == 0)) {
      local_64[0] = 8;
      local_64[1] = 7;
      local_4._0_1_ = 6;
      FUN_00520580(this_00,local_64);
      local_4._0_1_ = 1;
      FUN_00619730();
    }
    if (((iVar3 != 0) && (*(int *)((int)this_00 + 4) == -1)) && (iVar5 == 0)) {
      local_64[0] = 8;
      local_64[1] = 5;
      local_4._0_1_ = 7;
      FUN_00520580(this_00,local_64);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_64 + 2);
  ExceptionList = pvStack_c;
  return iVar3;
}

