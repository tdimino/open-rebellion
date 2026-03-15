
bool __thiscall
FUN_00545540(void *this,uint *param_1,undefined4 param_2,undefined4 param_3,int param_4,
            void *param_5)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  void *pvVar3;
  undefined3 extraout_var_00;
  void *this_00;
  bool bVar4;
  uint *puVar5;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647568;
  local_c = ExceptionList;
  bVar4 = true;
  ExceptionList = &local_c;
  bVar1 = FUN_005438a0((int *)param_1);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar2 = FUN_00549910(this,param_5);
    bVar4 = iVar2 != 0;
  }
  if (param_4 == 0) {
    if ((*param_1 >> 0x18 < 0x10) || (0x3f < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_00525750(local_2c,this);
      local_4 = 0;
      FUN_00513120((int)local_2c);
      while (local_10 != 0) {
        puVar5 = param_1;
        pvVar3 = (void *)FUN_0052bed0((int)local_2c);
        bVar1 = FUN_00520cb0(pvVar3,(int *)puVar5);
        if (CONCAT31(extraout_var_00,bVar1) != 0) {
          pvVar3 = param_5;
          this_00 = (void *)FUN_0052bed0((int)local_2c);
          iVar2 = FUN_00522480(this_00,pvVar3);
          if ((iVar2 == 0) || (!bVar4)) {
            bVar4 = false;
          }
          else {
            bVar4 = true;
          }
        }
        FUN_005130d0((int)local_2c);
      }
      local_4 = 0xffffffff;
      FUN_005257f0(local_2c);
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

