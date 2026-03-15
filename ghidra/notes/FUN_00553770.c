
int __cdecl FUN_00553770(undefined4 param_1,uint param_2,void *param_3)

{
  void *this;
  short sVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  uint local_2c;
  undefined4 local_28;
  uint uStack_24;
  undefined4 uStack_20;
  undefined4 auStack_1c [2];
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_00649068;
  local_c = ExceptionList;
  iVar4 = 1;
  local_2c = 1;
  local_28 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_2c);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (*(int *)((int)this + 4) == -1) {
    sVar1 = FUN_005f6340(param_2);
    if (sVar1 == 0) {
      local_2c = 1;
      local_28 = 0x16;
      local_4 = 1;
      FUN_00520580(this,&local_2c);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    if (*(int *)((int)this + 4) == -1) {
      param_3 = (void *)0x1;
      piVar2 = (int *)FUN_0041c210(param_2);
      uVar3 = param_2;
      while (((piVar2 != (int *)0x0 && (iVar4 != 0)) && (*(int *)((int)this + 4) == -1))) {
        iVar4 = (**(code **)(*piVar2 + 0x18))(this);
        if (iVar4 != 0) {
          if (*(int *)((int)this + 4) == -1) {
            param_2 = 0;
            FUN_00520570(auStack_1c);
            uStack_24 = 1;
            uStack_20 = 0x12;
            local_4._0_1_ = 3;
            local_4._1_3_ = 0;
            iVar4 = FUN_00504e60((uint *)(piVar2 + 0xf),&param_2,&uStack_24,auStack_1c);
            local_4 = CONCAT31(local_4._1_3_,2);
            FUN_00619730();
            if (iVar4 != 0) {
              if (param_2 == 0) {
                FUN_00520580(this,auStack_1c);
              }
              else {
                uVar3 = *(uint *)(param_2 + 0x24) >> 6 & 3;
              }
            }
            local_4 = 0xffffffff;
            FUN_00619730();
          }
          if ((iVar4 != 0) && (*(int *)((int)this + 4) == -1)) {
            if (param_3 != (void *)0x0) {
              param_3 = (void *)0x0;
              local_2c = uVar3;
            }
            if (uVar3 != local_2c) {
              uStack_14 = 1;
              uStack_10 = 0x14;
              local_4 = 4;
              FUN_00520580(this,&uStack_14);
              local_4 = 0xffffffff;
              FUN_00619730();
            }
          }
        }
        piVar2 = (int *)piVar2[4];
      }
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

