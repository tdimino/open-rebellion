
void __cdecl FUN_00417520(uint param_1)

{
  LPCSTR pCVar1;
  int iVar2;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ae70;
  local_c = ExceptionList;
  if ((param_1 < 0x10) && (DAT_006a8470 != 0)) {
    ExceptionList = &local_c;
    FUN_00610c80(0x6b1478);
    if ((DAT_006b1488 >> 5 & 1) != 0) {
      FUN_00417640();
    }
    DAT_006b147c = param_1 + 1;
    FUN_005f2f90(local_24,0x6b1498);
    local_4 = 0;
    FUN_005f35b0(local_18,(&PTR_s_MDATA_300_006a8478)[param_1]);
    local_4._0_1_ = 1;
    FUN_005f30d0(local_24,(int)local_18);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(local_18);
    iVar2 = 0;
    pCVar1 = (LPCSTR)FUN_00583c40((int)local_24);
    FUN_00610c40(0x6b1478,pCVar1,iVar2);
    FUN_00610cb0(0x6b1478,DAT_006b14a8);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_24);
  }
  ExceptionList = local_c;
  return;
}

