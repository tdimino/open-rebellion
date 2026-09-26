
uint __fastcall FUN_00523c60(int *param_1)

{
  uint uVar1;
  uint *puVar2;
  uint local_40;
  undefined4 local_3c [11];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643b68;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_004f9700(param_1);
  if (uVar1 != 0) {
    FUN_00525bb0(local_3c,param_1);
    local_4 = 0;
    FUN_00525930((int)local_3c);
    do {
      if (local_10 == 0) break;
      if (uVar1 != 0) {
        puVar2 = FUN_004025b0(param_1,&local_40);
        uVar1 = (uint)(*(uint *)(local_10 + 0x68) == *puVar2);
        FUN_00619730();
        if ((((uVar1 != 0) &&
             (uVar1 = (uint)((((byte)param_1[9] ^ (byte)*(undefined4 *)(local_10 + 0x24)) & 0xc0) ==
                            0), uVar1 != 0)) &&
            (uVar1 = *(uint *)(local_10 + 0x50) >> 6 & 1, uVar1 != 0)) &&
           ((uVar1 = (uint)((*(byte *)(local_10 + 0x78) & 0xc) == 0), uVar1 != 0 &&
            ((*(uint *)(local_10 + 0x50) & 0x20) != 0)))) {
          uVar1 = *(uint *)(local_10 + 0x50) >> 0xb & 1;
        }
      }
      FUN_005258f0((int)local_3c);
    } while (uVar1 != 0);
    local_4 = 0xffffffff;
    FUN_00525c50(local_3c);
  }
  ExceptionList = local_c;
  return uVar1;
}

