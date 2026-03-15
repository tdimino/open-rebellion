
uint FUN_00526c00(void)

{
  uint uVar1;
  undefined4 *puVar2;
  
  uVar1 = 1;
  if (DAT_006b9000 == 0) {
    DAT_006b9000 = 1;
    uVar1 = FUN_004f2a10(0,&DAT_006b8ff8);
    if (uVar1 != 0) {
      puVar2 = FUN_00520690(&DAT_006b8ff8,0x201,1);
      uVar1 = (uint)(puVar2 != (undefined4 *)0x0);
      if (uVar1 != 0) {
        puVar2 = FUN_00520690(&DAT_006b8ff8,0x202,1);
        uVar1 = (uint)(puVar2 != (undefined4 *)0x0);
        if (uVar1 != 0) {
          puVar2 = FUN_00520690(&DAT_006b8ff8,0x204,1);
          uVar1 = (uint)(puVar2 != (undefined4 *)0x0);
        }
      }
    }
  }
  return uVar1;
}

