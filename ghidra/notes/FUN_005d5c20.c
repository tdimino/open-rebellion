
void __fastcall FUN_005d5c20(undefined4 *param_1)

{
  undefined4 *this;
  int iVar1;
  int iVar2;
  undefined4 *puVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00654843;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066c990;
  this = param_1 + 1;
  local_4 = 1;
  iVar1 = FUN_005f5060((int)this);
  do {
    if (iVar1 == 0) {
      local_4 = local_4 & 0xffffff00;
      FUN_005d6350(param_1 + 3);
      *param_1 = &PTR_FUN_0066c998;
      local_4 = 0xffffffff;
      *this = &PTR_FUN_0066c9a0;
      FUN_005f4f00(this);
      ExceptionList = pvStack_c;
      return;
    }
    iVar2 = (**(code **)(**(int **)(iVar1 + 0x1c) + 8))(*(int **)(iVar1 + 0x1c));
    if (iVar2 != 0) {
      iVar2 = (**(code **)(**(int **)(iVar1 + 0x1c) + 8))(*(int **)(iVar1 + 0x1c));
      while (iVar2 != 0) {
        iVar2 = (**(code **)(**(int **)(iVar1 + 0x1c) + 8))(*(int **)(iVar1 + 0x1c));
      }
    }
    puVar3 = (undefined4 *)FUN_005f5060((int)this);
    if (puVar3 != (undefined4 *)0x0) {
      do {
        if (puVar3[7] == *(int *)(iVar1 + 0x1c)) goto LAB_005d5ca1;
        puVar3 = (undefined4 *)puVar3[4];
      } while (puVar3 != (undefined4 *)0x0);
    }
    puVar3 = (undefined4 *)0x0;
LAB_005d5ca1:
    if (puVar3 != (undefined4 *)0x0) {
      FUN_005f4fa0(this,(int)puVar3);
      (**(code **)*puVar3)(1);
    }
    iVar1 = FUN_005f5060((int)this);
  } while( true );
}

