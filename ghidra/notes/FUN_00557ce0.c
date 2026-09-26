
uint FUN_00557ce0(void)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006bb358 == 0) {
    DAT_006bb358 = 1;
    iVar2 = FUN_00558380(0,&DAT_006bb350);
    uVar1 = 0;
    if (iVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006bb350,0x201,1);
      uVar1 = 0;
      if (puVar3 != (undefined4 *)0x0) {
        puVar3 = FUN_00520690(&DAT_006bb350,0x202,1);
        uVar1 = 0;
        if (puVar3 != (undefined4 *)0x0) {
          puVar3 = FUN_00520690(&DAT_006bb350,0x204,1);
          uVar1 = 0;
          if (puVar3 != (undefined4 *)0x0) {
            puVar3 = FUN_00520690(&DAT_006bb350,0x200,1);
            uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
          }
        }
      }
    }
  }
  return uVar1;
}

