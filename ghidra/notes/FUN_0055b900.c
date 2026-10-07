
uint FUN_0055b900(void)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006bb4e0 == 0) {
    DAT_006bb4e0 = 1;
    uVar2 = FUN_0052aeb0(0,&DAT_006bb4e8);
    uVar1 = 0;
    if (uVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006bb4e8,0x210,1);
      uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
    }
    DAT_006bb4e0 = 1;
  }
  return uVar1;
}

