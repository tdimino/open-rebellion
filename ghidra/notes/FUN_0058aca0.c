
undefined4 FUN_0058aca0(int param_1,uint *param_2)

{
  uint *this;
  int iVar1;
  uint *puVar2;
  void *this_00;
  uint *puVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fd18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece80(param_2);
  iVar5 = 0;
  for (iVar1 = thunk_FUN_005f5060(param_1); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)) {
    if (*(int *)(iVar1 + 0x1c) == 0) {
      iVar5 = iVar5 + 1;
    }
  }
  if (iVar5 != 0) {
    puVar2 = (uint *)FUN_0053e290(iVar5 + -1);
    param_2 = (uint *)0x0;
    this_00 = (void *)thunk_FUN_005f5060(param_1);
    while ((this_00 != (void *)0x0 && (puVar3 = FUN_004ece40(this), puVar3 == (uint *)0x0))) {
      if (*(int *)((int)this_00 + 0x1c) == 0) {
        if (param_2 == puVar2) {
          *(undefined4 *)((int)this_00 + 0x1c) = DAT_0066a7e4;
          puVar4 = (undefined4 *)FUN_00403040(this_00,local_10);
          local_4 = 0;
          FUN_004f26d0(this,puVar4);
          local_4 = 0xffffffff;
          FUN_00619730();
        }
        param_2 = (uint *)((int)param_2 + 1);
      }
      this_00 = *(void **)((int)this_00 + 0x10);
    }
  }
  ExceptionList = local_c;
  return 1;
}

