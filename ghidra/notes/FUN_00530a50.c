
undefined4 __cdecl FUN_00530a50(uint param_1,void *param_2)

{
  char *pcVar1;
  void *pvVar2;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645558;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f35b0(local_18,&DAT_006b120c);
  local_4 = 0;
  FUN_005f3090(param_2,(int)local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa594;
  if ((param_1 & 1) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 1;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa58c;
  if ((param_1 & 0x10) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 2;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa588;
  if ((param_1 & 0x20) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 3;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa584;
  if ((param_1 & 0x100) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 4;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa580;
  if ((param_1 & 0x200) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 5;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa57c;
  if ((param_1 & 0x1000) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 6;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa578;
  if ((param_1 & 0x2000) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 7;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa574;
  if ((param_1 & 0x100000) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 8;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa570;
  if ((param_1 & 0x1000000) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 9;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  pcVar1 = &DAT_006aa56c;
  if ((param_1 & 0x10000000) == 0) {
    pcVar1 = &DAT_006aa590;
  }
  pvVar2 = FUN_005f35b0(local_18,pcVar1);
  local_4 = 10;
  FUN_005f30d0(param_2,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return 1;
}

