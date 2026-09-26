
void __fastcall FUN_005d6930(int param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  
  puVar1 = (undefined4 *)(param_1 + 0x124);
  piVar2 = (int *)(param_1 + 0x128);
  *puVar1 = 0;
  *piVar2 = 0;
  DAT_006bcbb4 = Direct3DRMCreate();
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  DAT_006bcbb4 = (*(code *)**(undefined4 **)*puVar1)((undefined4 *)*puVar1,&DAT_0066e448,piVar2);
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  piVar2 = (int *)*piVar2;
  if ((piVar2 != (int *)0x0) && (puVar1 != (undefined4 *)0x0)) {
    DAT_006bcbb4 = (**(code **)(*piVar2 + 100))(piVar2,puVar1);
    if (DAT_006bcbb4 != 0) {
      FUN_005d93f0(0,DAT_006bcbb4);
    }
  }
  return;
}

