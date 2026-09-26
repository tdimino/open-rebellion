
int __thiscall
FUN_00545670(void *this,uint *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar4;
  int iVar5;
  uint *puVar6;
  undefined4 uVar7;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00647590;
  local_c = ExceptionList;
  iVar5 = 1;
  uVar2 = *param_1 >> 0x18;
  bVar1 = false;
  local_4 = 0;
  if ((uVar2 < 0x98) || (ExceptionList = &local_c, 0x9f < uVar2)) {
    if ((0x8f < uVar2) && (uVar2 < 0x98)) {
      bVar1 = true;
    }
    ExceptionList = &local_c;
    FUN_00619730();
    if (!bVar1) {
      if ((*param_1 >> 0x18 < 0x10) || (0x3f < *param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        bVar1 = false;
        goto LAB_00545726;
      }
    }
  }
  bVar1 = true;
LAB_00545726:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    FUN_00525750(local_2c,this);
    local_4 = 1;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      puVar6 = param_1;
      pvVar3 = (void *)FUN_0052bed0((int)local_2c);
      bVar1 = FUN_00520c70(pvVar3,(int *)puVar6);
      if (CONCAT31(extraout_var,bVar1) == 0) {
        puVar6 = param_1;
        pvVar3 = (void *)FUN_0052bed0((int)local_2c);
        bVar1 = FUN_00520cb0(pvVar3,(int *)puVar6);
        if (CONCAT31(extraout_var_00,bVar1) != 0) goto LAB_00545793;
      }
      else {
LAB_00545793:
        uVar7 = param_4;
        pvVar3 = (void *)FUN_0052bed0((int)local_2c);
        iVar4 = FUN_00522480(pvVar3,uVar7);
        if ((iVar4 == 0) || (iVar5 == 0)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_005257f0(local_2c);
  }
  ExceptionList = local_c;
  return iVar5;
}

