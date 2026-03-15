
undefined4 __cdecl FUN_0053ed20(int param_1,void *param_2)

{
  undefined4 uVar1;
  void *this;
  int iVar2;
  int iVar3;
  void *this_00;
  uint *puVar4;
  void *this_01;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006468a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_005f2f50(local_18);
  this = param_2;
  local_4 = 0;
  FUN_005f3090(param_2,iVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  FUN_005f35b0(local_24,s__IDs__006aad70);
  local_4 = 1;
  FUN_005f30d0(this,(int)local_24);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_24);
  iVar2 = param_1;
  iVar3 = thunk_FUN_005f5060(param_1);
  if (iVar3 == 0) {
    FUN_005f35b0(local_24,s_<none>_006aad68);
    local_4 = 2;
    FUN_005f30d0(this,(int)local_24);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_24);
  }
  param_2 = (void *)0x1;
  for (this_00 = (void *)thunk_FUN_005f5060(iVar2); this_00 != (void *)0x0;
      this_00 = *(void **)((int)this_00 + 0x10)) {
    if (param_2 == (void *)0x0) {
      FUN_005f35b0(local_24,&DAT_006a8f48);
      local_4 = 3;
      FUN_005f30d0(this,(int)local_24);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_24);
    }
    else {
      param_2 = (void *)0x0;
    }
    FUN_005f35b0(local_18,&DAT_006a8f3c);
    uVar1 = *(undefined4 *)((int)this_00 + 0x1c);
    local_4 = 4;
    puVar4 = (uint *)FUN_00403040(this_00,&param_1);
    local_4._0_1_ = 5;
    this_01 = FUN_005f31f0(this,*puVar4 & 0xffffff);
    FUN_005f30d0(this_01,(int)local_18);
    FUN_005f31a0(this_01,uVar1);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
  }
  FUN_005f35b0(local_18,&DAT_006aa5d0);
  local_4 = 6;
  FUN_005f30d0(this,(int)local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return 1;
}

