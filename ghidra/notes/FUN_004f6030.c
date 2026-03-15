
void __thiscall FUN_004f6030(void *this,void *param_1,undefined4 param_2)

{
  int iVar1;
  bool bVar2;
  undefined *puVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  void *pvVar4;
  undefined4 *puVar5;
  undefined2 local_60;
  undefined2 local_5e;
  undefined2 local_5c;
  undefined2 local_5a;
  undefined2 local_58;
  undefined2 local_56;
  undefined4 local_54 [3];
  undefined4 local_48 [3];
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063fb00;
  local_c = ExceptionList;
  iVar1 = (int)this + -0x30;
  ExceptionList = &local_c;
  puVar3 = FUN_004f62d0(iVar1);
  FUN_005f2f90(local_48,(int)puVar3);
  local_4 = 0;
  switch(param_2) {
  case 1:
  case 2:
  case 4:
    puVar3 = FUN_004f62d0(iVar1);
    break;
  case 3:
    if (*(int *)((int)this + -4) == 0) goto switchD_004f607e_caseD_5;
    puVar3 = (undefined *)(*(int *)((int)this + -4) + 0x34);
    break;
  default:
    goto switchD_004f607e_caseD_5;
  case 0x20:
  case 0x21:
    bVar2 = FUN_005f34a0((int)local_48);
    if (CONCAT31(extraout_var,bVar2) == 0) {
      local_5e = DAT_0065d424;
      local_60 = 0x4da2;
      pvVar4 = FUN_005f2fc0(local_30,&local_60);
      local_4._0_1_ = 2;
      FUN_005f30d0(param_1,(int)pvVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f2ff0(local_30);
      puVar3 = FUN_004f62d0(iVar1);
    }
    else {
      param_2 = CONCAT22(DAT_0065d424,0x4da1);
      pvVar4 = FUN_005f2fc0(local_3c,&param_2);
      local_4._0_1_ = 1;
      FUN_005f30d0(param_1,(int)pvVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f2ff0(local_3c);
      puVar3 = FUN_004f62d0(iVar1);
    }
    break;
  case 0x22:
    if (*(int *)((int)this + -4) != 0) {
      FUN_005f2f90(local_54,*(int *)((int)this + -4) + 0x34);
      local_4._0_1_ = 3;
      bVar2 = FUN_005f34a0((int)local_54);
      if (CONCAT31(extraout_var_00,bVar2) == 0) {
        local_58 = 0x4da2;
        local_56 = DAT_0065d424;
        pvVar4 = FUN_005f2fc0(local_18,&local_58);
        local_4._0_1_ = 5;
        FUN_005f30d0(param_1,(int)pvVar4);
        puVar5 = local_18;
      }
      else {
        local_5c = 0x4da1;
        local_5a = DAT_0065d424;
        pvVar4 = FUN_005f2fc0(local_24,&local_5c);
        local_4._0_1_ = 4;
        FUN_005f30d0(param_1,(int)pvVar4);
        puVar5 = local_24;
      }
      local_4._0_1_ = 3;
      FUN_005f2ff0(puVar5);
      FUN_005f30d0(param_1,(int)local_54);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f2ff0(local_54);
    }
    goto switchD_004f607e_caseD_5;
  }
  FUN_005f30d0(param_1,(int)puVar3);
switchD_004f607e_caseD_5:
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_48);
  ExceptionList = local_c;
  return;
}

