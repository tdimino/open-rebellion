
void __thiscall FUN_004362b0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  int iVar1;
  int iVar2;
  void *unaff_EBP;
  undefined4 unaff_ESI;
  undefined4 unaff_EDI;
  undefined4 *puStack_34;
  undefined4 uStack_30;
  uint local_1c;
  undefined4 local_18;
  undefined4 *local_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d530;
  pvStack_c = ExceptionList;
  uStack_30 = 0x4362d7;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  uStack_30 = 0x4362e8;
  FUN_004ece30(&local_1c);
  puStack_34 = &local_18;
  uStack_30 = param_3;
  local_4 = CONCAT31(local_4._1_3_,1);
  iVar1 = (**(code **)(**(int **)((int)this + 0x10) + 0x3c))(&local_14);
  iVar2 = (**(code **)(**(int **)((int)this + 0x14) + 0x3c))(&local_1c,&stack0xffffffd8,param_1);
  if (iVar1 != 0) {
    (**(code **)(**(int **)((int)this + 0x14) + 0x48))(unaff_EDI,&uStack_30,iVar1,param_3);
  }
  if (iVar2 != 0) {
    (**(code **)(**(int **)((int)this + 0x10) + 0x48))(unaff_ESI,&puStack_34,iVar2,puStack_8);
  }
  if ((iVar1 == 0) || (iVar2 == 0)) {
    *local_14 = 0;
    *puStack_10 = 0;
  }
  else {
    *local_14 = 1;
    if ((iVar1 == 1) && (iVar2 == 1)) {
      *puStack_10 = 1;
    }
    else {
      *puStack_10 = 0;
    }
  }
  local_1c = local_1c & 0xffffff00;
  FUN_00619730();
  local_1c = 0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_EBP;
  return;
}

