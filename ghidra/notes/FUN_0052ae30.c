
uint FUN_0052ae30(void)

{
  uint uVar1;
  undefined4 *puVar2;
  
  uVar1 = 1;
  if (DAT_006b9008 == 0) {
    DAT_006b9008 = 1;
    uVar1 = FUN_004fad70(0,&DAT_006b9018);
    if (uVar1 != 0) {
      puVar2 = FUN_00520690(&DAT_006b9018,0x213,1);
      uVar1 = (uint)(puVar2 != (undefined4 *)0x0);
      if (uVar1 != 0) {
        puVar2 = FUN_00520690(&DAT_006b9018,0x214,1);
        uVar1 = (uint)(puVar2 != (undefined4 *)0x0);
        if (uVar1 != 0) {
          puVar2 = FUN_00520690(&DAT_006b9018,0x216,1);
          uVar1 = (uint)(puVar2 != (undefined4 *)0x0);
        }
      }
    }
    DAT_006b9008 = 1;
  }
  return uVar1;
}

