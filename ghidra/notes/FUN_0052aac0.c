
uint __thiscall FUN_0052aac0(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  uint *puVar3;
  void *this_00;
  uint uVar4;
  int *piVar5;
  bool bVar6;
  uint local_1c;
  void *local_18;
  undefined1 local_14 [4];
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006449f0;
  local_c = ExceptionList;
  bVar6 = true;
  local_18 = this;
  if (*(int *)((int)this + 0x58) == 0) {
    ExceptionList = &local_c;
    iVar2 = thunk_FUN_005f5060(*(int *)((int)this + 0x54) + 0x18);
    if (iVar2 != 0) {
      ExceptionList = local_c;
      return 0;
    }
    piVar5 = (int *)FUN_004ece30(&local_10);
    local_4 = 3;
    uVar4 = FUN_00529770(this,piVar5,param_1);
  }
  else {
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_0052be30(this);
    if ((pvVar1 != (void *)0x0) && (iVar2 = FUN_00528890(this,pvVar1), iVar2 != 0)) {
      ExceptionList = local_c;
      return 1;
    }
    FUN_004ece30(&local_1c);
    local_4._0_1_ = 0;
    local_4._1_3_ = 0;
    pvVar1 = (void *)thunk_FUN_005f5060(*(int *)((int)this + 0x54) + 0x18);
    do {
      if ((pvVar1 == (void *)0x0) || (puVar3 = FUN_004ece40(&local_1c), puVar3 != (uint *)0x0))
      break;
      puVar3 = FUN_00403040(pvVar1,local_14);
      local_4._0_1_ = 1;
      this_00 = (void *)FUN_00504dc0(puVar3);
      local_4._0_1_ = 0;
      FUN_00619730();
      bVar6 = this_00 != (void *)0x0;
      if ((bVar6) && (iVar2 = FUN_00528890(local_18,this_00), iVar2 != 0)) {
        puVar3 = FUN_004025b0(this_00,&local_10);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_1c,puVar3);
        local_4._0_1_ = 0;
        FUN_00619730();
      }
      pvVar1 = *(void **)((int)pvVar1 + 0x10);
      this = local_18;
    } while (bVar6);
    uVar4 = 0;
    if (bVar6) {
      uVar4 = FUN_00529770(this,(int *)&local_1c,param_1);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar4;
}

