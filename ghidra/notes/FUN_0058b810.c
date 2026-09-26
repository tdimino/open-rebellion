
void __thiscall FUN_0058b810(void *this,uint param_1,uint param_2,undefined4 param_3)

{
  undefined4 *puVar1;
  char *pcVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fdff;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x5c);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00595150(puVar1);
  }
  local_4 = 0xffffffff;
  if ((puVar1 != (undefined4 *)0x0) && (puVar1[7] != 0)) {
    pcVar2 = (char *)FUN_004fd320();
    iVar3 = FUN_00595210(puVar1,param_1,param_2,pcVar2);
    if (iVar3 != 0) {
      FUN_005f5600(puVar1,param_3);
      FUN_005f5440(*(void **)((int)this + 8),puVar1);
    }
  }
  ExceptionList = local_c;
  return;
}

