
uint FUN_00536bc0(void)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006b9030 == 0) {
    DAT_006b9030 = 1;
    iVar2 = FUN_004f2a10(0,&DAT_006b9028);
    uVar1 = 0;
    if (iVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006b9028,0x201,1);
      uVar1 = 0;
      if (puVar3 != (undefined4 *)0x0) {
        puVar3 = FUN_00520690(&DAT_006b9028,0x202,1);
        uVar1 = 0;
        if (puVar3 != (undefined4 *)0x0) {
          puVar3 = FUN_00520690(&DAT_006b9028,0x242,1);
          uVar1 = 0;
          if (puVar3 != (undefined4 *)0x0) {
            puVar3 = FUN_00520690(&DAT_006b9028,0x240,1);
            uVar1 = 0;
            if (puVar3 != (undefined4 *)0x0) {
              puVar3 = FUN_00520690(&DAT_006b9028,0x204,1);
              uVar1 = 0;
              if (puVar3 != (undefined4 *)0x0) {
                puVar3 = FUN_00520690(&DAT_006b9028,0x241,1);
                uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
              }
            }
          }
        }
      }
    }
  }
  return uVar1;
}

