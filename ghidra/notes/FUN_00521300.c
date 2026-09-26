
undefined4 FUN_00521300(int param_1,uint *param_2,undefined4 *param_3)

{
  bool bVar1;
  void *this;
  uint *puVar2;
  int *this_00;
  int iVar3;
  undefined4 *puVar4;
  int iVar5;
  int local_24;
  undefined1 local_20 [4];
  uint uStack_1c;
  uint uStack_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006437a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece80(param_2);
  FUN_004fcdd0(param_3);
  local_24 = 0;
  iVar5 = 1;
  for (this = (void *)thunk_FUN_005f5060(param_1); this != (void *)0x0;
      this = *(void **)((int)this + 0x10)) {
    puVar2 = FUN_00403040(this,&param_1);
    local_14 = 0x30;
    local_10 = 0x40;
    bVar1 = false;
    local_4 = 0;
    if ((0x2f < *puVar2 >> 0x18) && (*puVar2 >> 0x18 < 0x40)) {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      puVar2 = FUN_00403040(this,local_20);
      local_4 = 1;
      this_00 = (int *)FUN_00505d40(puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((((this_00 != (int *)0x0) && ((*(byte *)(this_00 + 0x1e) & 0xc) == 0)) &&
          ((*(byte *)(this_00 + 0x14) & 8) == 0)) &&
         ((iVar3 = (**(code **)(*this_00 + 0x1d4))(), iVar3 == 0 &&
          (((iVar3 = FUN_004ece60(param_2), iVar3 == 0 ||
            (iVar3 = (**(code **)(*this_00 + 0x1fc))(), iVar5 < iVar3)) ||
           ((iVar3 = (**(code **)(*this_00 + 0x1fc))(), iVar3 == iVar5 &&
            (iVar3 = (**(code **)(*this_00 + 500))(), local_24 < iVar3)))))))) {
        puVar2 = FUN_004025b0(this_00,&uStack_1c);
        local_4 = 2;
        FUN_004f26d0(param_2,puVar2);
        local_4 = 0xffffffff;
        FUN_00619730();
        if ((void *)this_00[0xb] != (void *)0x0) {
          puVar4 = (undefined4 *)FUN_00402e40((void *)this_00[0xb],&uStack_18);
          local_4 = 3;
          FUN_004f26d0(param_3,puVar4);
          local_4 = 0xffffffff;
          FUN_00619730();
        }
        local_24 = (**(code **)(*this_00 + 500))();
        iVar5 = (**(code **)(*this_00 + 0x1fc))();
      }
    }
  }
  ExceptionList = local_c;
  return 1;
}

