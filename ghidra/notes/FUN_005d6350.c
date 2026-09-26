
void __fastcall FUN_005d6350(undefined4 *param_1)

{
  undefined4 *this;
  int iVar1;
  undefined4 *puVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00654943;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066c9b0;
  this = param_1 + 1;
  local_4 = 1;
  iVar1 = FUN_005f5060((int)this);
  do {
    if (iVar1 == 0) {
      local_4 = local_4 & 0xffffff00;
      FUN_005d6090(param_1 + 3);
      *param_1 = &PTR_FUN_0066c9d0;
      local_4 = 0xffffffff;
      *this = &PTR_FUN_0066c9d8;
      FUN_005f4f00(this);
      ExceptionList = pvStack_c;
      return;
    }
    (**(code **)(**(int **)(iVar1 + 0x24) + 8))(*(int **)(iVar1 + 0x24));
    puVar2 = (undefined4 *)FUN_005f5060((int)this);
    if (puVar2 != (undefined4 *)0x0) {
      do {
        if (puVar2[9] == *(int *)(iVar1 + 0x24)) goto LAB_005d63b2;
        puVar2 = (undefined4 *)puVar2[4];
      } while (puVar2 != (undefined4 *)0x0);
    }
    puVar2 = (undefined4 *)0x0;
LAB_005d63b2:
    if (puVar2 != (undefined4 *)0x0) {
      FUN_005f4fa0(this,(int)puVar2);
      (**(code **)*puVar2)(1);
    }
    iVar1 = FUN_005f5060((int)this);
  } while( true );
}

