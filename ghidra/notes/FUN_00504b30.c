
uint FUN_00504b30(void)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006b2b68 == 0) {
    DAT_006b2b68 = 1;
    iVar2 = FUN_00558380(0,&DAT_006b2b70);
    uVar1 = 0;
    if (iVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006b2b70,0x201,1);
      uVar1 = 0;
      if (puVar3 != (undefined4 *)0x0) {
        puVar3 = FUN_00520690(&DAT_006b2b70,0x202,1);
        uVar1 = 0;
        if (puVar3 != (undefined4 *)0x0) {
          puVar3 = FUN_00520690(&DAT_006b2b70,0x204,1);
          uVar1 = 0;
          if (puVar3 != (undefined4 *)0x0) {
            puVar3 = FUN_00520690(&DAT_006b2b70,0x200,1);
            uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
          }
        }
      }
    }
  }
  return uVar1;
}

