
uint __cdecl FUN_005185c0(int *param_1,int *param_2)

{
  bool bVar1;
  int *piVar2;
  uint uVar3;
  uint *puVar4;
  int iVar5;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642ab8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_004ece30(&param_2);
  local_4 = 0;
  uVar3 = (**(code **)(*param_1 + 0xc))(&param_2);
  if (uVar3 != 0) {
    puVar4 = FUN_004ece40((uint *)&param_1);
    if (puVar4 != (uint *)0x0) {
      if (((uint)param_1 >> 0x18 < 0x90) || (0x97 < (uint)param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        iVar5 = FUN_00505f60((uint *)&param_1);
        uVar3 = (uint)(iVar5 != 0);
        *piVar2 = iVar5;
      }
    }
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_10;
  return uVar3;
}

