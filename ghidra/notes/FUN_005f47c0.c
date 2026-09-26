
undefined4 __thiscall FUN_005f47c0(void *this,int param_1,undefined4 *param_2)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006556e0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f3de0(local_24,param_1);
  local_4 = 0;
  iVar1 = FUN_005f3fa0((int)local_24);
  do {
    if (iVar1 == 0) {
      uVar3 = FUN_005f4720(this,param_1,param_2);
      local_4 = 0xffffffff;
      FUN_005f3e00(local_24);
      ExceptionList = local_c;
      return uVar3;
    }
    FUN_005f42c0(local_24);
    iVar1 = FUN_005f3fa0((int)local_24);
    if (iVar1 != 0) {
      FUN_005f2f50(local_18);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005f4350((int)local_24,local_18);
      for (iVar1 = *(int *)((int)this + 8); iVar1 != 0; iVar1 = *(int *)(iVar1 + 8)) {
        iVar2 = FUN_005f3390((void *)(iVar1 + 0xc),(int)local_18);
        if (iVar2 == 0) {
          local_4 = local_4 & 0xffffff00;
          *param_2 = *(undefined4 *)(iVar1 + 0x18);
          FUN_005f2ff0(local_18);
          local_4 = 0xffffffff;
          FUN_005f3e00(local_24);
          ExceptionList = local_c;
          return 1;
        }
      }
      local_4 = local_4 & 0xffffff00;
      FUN_005f2ff0(local_18);
    }
    FUN_005f3e50((int)local_24);
    iVar1 = FUN_005f3fa0((int)local_24);
  } while( true );
}

