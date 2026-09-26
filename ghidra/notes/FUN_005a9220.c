
undefined4 __thiscall
FUN_005a9220(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5,undefined4 param_6,undefined4 param_7)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652dce;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x7c);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar3 = (undefined4 *)0x0;
  }
  else {
    FUN_005a6210(puVar1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005b82a0(puVar1 + 7);
    puVar1[0x13] = &PTR_FUN_0066bacc;
    puVar1[0x14] = 0;
    puVar1[0x15] = &PTR_FUN_0066bac8;
    puVar1[0x16] = 0;
    puVar1[0x17] = param_1;
    puVar1[0x18] = param_2;
    puVar1[0x19] = param_5;
    puVar1[0x1a] = param_6;
    *puVar1 = &PTR_FUN_0066c3c0;
    puVar1[7] = &PTR_LAB_0066c3b8;
    puVar1[0x1c] = param_3;
    puVar1[0x1d] = param_4;
    puVar1[0x1b] = 0x42c80000;
    puVar1[9] = 0;
    puVar1[10] = 0;
    puVar1[0xb] = param_7;
    puVar1[0x1e] = 0;
    puVar3 = puVar1;
  }
  local_4 = 0xffffffff;
  (**(code **)(*(int *)this + 0x50))(puVar3);
  if (DAT_006bc470 == (undefined4 *)0x0) {
    puVar2 = (undefined4 *)FUN_00618b70(0x48);
    puStack_8 = (undefined1 *)0x2;
    if (puVar2 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar2);
    }
  }
  DAT_006bc470[0x11] = puVar3;
  ExceptionList = puVar1;
  return puVar3[6];
}

