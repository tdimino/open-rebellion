
int __thiscall FUN_00545c30(void *this,uint *param_1,undefined4 param_2,undefined4 param_3)

{
  bool bVar1;
  void *pvVar2;
  undefined3 extraout_var;
  int iVar3;
  int iVar4;
  uint *puVar5;
  undefined4 uVar6;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647608;
  local_c = ExceptionList;
  iVar4 = 1;
  if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    FUN_0056d1b0(local_2c,this);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      puVar5 = param_1;
      pvVar2 = (void *)FUN_0052bed0((int)local_2c);
      bVar1 = FUN_00520c90(pvVar2,(int *)puVar5);
      if (CONCAT31(extraout_var,bVar1) != 0) {
        uVar6 = param_3;
        pvVar2 = (void *)FUN_0052bed0((int)local_2c);
        iVar3 = FUN_00522480(pvVar2,uVar6);
        if ((iVar3 == 0) || (iVar4 == 0)) {
          iVar4 = 0;
        }
        else {
          iVar4 = 1;
        }
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_0056d250(local_2c);
  }
  ExceptionList = local_c;
  return iVar4;
}

