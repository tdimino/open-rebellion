
/* WARNING: Removing unreachable block (ram,0x0051fdcd) */
/* WARNING: Removing unreachable block (ram,0x0051fdd3) */

undefined4 __fastcall FUN_0051fd30(int *param_1)

{
  int iVar1;
  int iVar2;
  undefined4 auStack_2c [2];
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c [3];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006433f0;
  pvStack_c = ExceptionList;
  local_24 = 1;
  iVar1 = param_1[7];
  local_20 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00541e70(local_1c);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_0051fcb0(param_1,local_1c);
  (**(code **)(*param_1 + 0x18))(&local_24);
  auStack_2c[0] = 0;
  iVar2 = 0;
  if (iVar1 != 0) {
    iVar2 = FUN_00553720(&local_20,param_1[0x10],(undefined4 *)&stack0xffffffd0,auStack_2c);
  }
  if (iVar2 != 0) {
    (**(code **)(*param_1 + 0x54))();
  }
  puStack_8 = (undefined1 *)((uint)puStack_8 & 0xffffff00);
  FUN_00541ea0();
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_10;
  return 0;
}

