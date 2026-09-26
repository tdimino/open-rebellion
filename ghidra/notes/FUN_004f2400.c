
uint FUN_004f2400(void)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006b2938 == 0) {
    DAT_006b2938 = 1;
    uVar2 = FUN_00536ca0(0,&DAT_006b2940);
    uVar1 = 0;
    if (uVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006b2940,0x260,1);
      uVar1 = 0;
      if (puVar3 != (undefined4 *)0x0) {
        puVar3 = FUN_00520690(&DAT_006b2940,0x261,1);
        uVar1 = 0;
        if (puVar3 != (undefined4 *)0x0) {
          puVar3 = FUN_00520690(&DAT_006b2940,0x262,1);
          uVar1 = 0;
          if (puVar3 != (undefined4 *)0x0) {
            puVar3 = FUN_00520690(&DAT_006b2940,0x263,1);
            uVar1 = 0;
            if (puVar3 != (undefined4 *)0x0) {
              puVar3 = FUN_00520690(&DAT_006b2940,0x268,1);
              uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
            }
          }
        }
      }
    }
  }
  return uVar1;
}

