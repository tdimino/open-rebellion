
void __fastcall FUN_005d5fc0(undefined4 *param_1)

{
  undefined4 *this;
  int iVar1;
  undefined4 *puVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_006548c3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066c9e8;
  this = param_1 + 1;
  local_4 = 1;
  iVar1 = FUN_005f5060((int)this);
  do {
    if (iVar1 == 0) {
      local_4 = local_4 & 0xffffff00;
      FUN_005d5d60(param_1 + 3);
      *param_1 = &PTR_FUN_0066ca08;
      local_4 = 0xffffffff;
      *this = &PTR_FUN_0066ca10;
      FUN_005f4f00(this);
      ExceptionList = pvStack_c;
      return;
    }
    (**(code **)(**(int **)(iVar1 + 0x1c) + 8))(*(int **)(iVar1 + 0x1c));
    puVar2 = (undefined4 *)FUN_005f5060((int)this);
    if (puVar2 != (undefined4 *)0x0) {
      do {
        if (puVar2[7] == *(int *)(iVar1 + 0x1c)) goto LAB_005d6022;
        puVar2 = (undefined4 *)puVar2[4];
      } while (puVar2 != (undefined4 *)0x0);
    }
    puVar2 = (undefined4 *)0x0;
LAB_005d6022:
    if (puVar2 != (undefined4 *)0x0) {
      FUN_005f4fa0(this,(int)puVar2);
      (**(code **)*puVar2)(1);
    }
    iVar1 = FUN_005f5060((int)this);
  } while( true );
}

