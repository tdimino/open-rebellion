
void __fastcall FUN_0040fe10(void *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a4b0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_3c);
  local_4 = 0;
  if (*(int *)((int)param_1 + 0x120) == 1) {
    FUN_005f35b0(local_24,&DAT_006a815c);
    local_4._0_1_ = 1;
    FUN_005f30d0(local_3c,(int)local_24);
    puVar2 = local_24;
LAB_0040fe9b:
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(puVar2);
  }
  else if (*(int *)((int)param_1 + 0x120) == 2) {
    FUN_005f35b0(local_30,&DAT_006a8160);
    local_4._0_1_ = 2;
    FUN_005f30d0(local_3c,(int)local_30);
    puVar2 = local_30;
    goto LAB_0040fe9b;
  }
  iVar1 = *(int *)((int)param_1 + 0x124);
  if (iVar1 == 1) {
    FUN_005f35b0(local_18,&DAT_006a8150);
    local_4._0_1_ = 3;
    FUN_005f30d0(local_3c,(int)local_18);
    local_4 = (uint)local_4._1_3_ << 8;
    puVar2 = local_18;
LAB_0040ff32:
    FUN_005f2ff0(puVar2);
  }
  else {
    if (iVar1 == 2) {
      FUN_005f35b0(local_30,&DAT_006a8154);
      local_4._0_1_ = 4;
      FUN_005f30d0(local_3c,(int)local_30);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar2 = local_30;
      goto LAB_0040ff32;
    }
    if (iVar1 == 3) {
      FUN_005f35b0(local_24,&DAT_006a8158);
      local_4._0_1_ = 5;
      FUN_005f30d0(local_3c,(int)local_24);
      local_4 = (uint)local_4._1_3_ << 8;
      puVar2 = local_24;
      goto LAB_0040ff32;
    }
  }
  if (*(int *)((int)param_1 + 0x128) == 1) {
    FUN_005f35b0(local_24,&DAT_006a8148);
    local_4._0_1_ = 6;
    FUN_005f30d0(local_3c,(int)local_24);
    puVar2 = local_24;
  }
  else {
    if (*(int *)((int)param_1 + 0x128) != 2) goto LAB_0040ff9c;
    FUN_005f35b0(local_18,&DAT_006a814c);
    local_4._0_1_ = 7;
    FUN_005f30d0(local_3c,(int)local_18);
    puVar2 = local_18;
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(puVar2);
LAB_0040ff9c:
  FUN_00410240(param_1,(int)local_3c);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_3c);
  ExceptionList = local_c;
  return;
}

