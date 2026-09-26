
undefined4 __cdecl FUN_00626870(int param_1,LCID param_2,LCTYPE param_3,char *param_4)

{
  byte bVar1;
  bool bVar2;
  size_t _Count;
  DWORD DVar3;
  int iVar4;
  LPSTR _Source;
  char *_Dest;
  uint uVar5;
  byte *pbVar6;
  CHAR local_80 [128];
  
  if (param_1 != 1) {
    if (param_1 != 0) {
      return 0xffffffff;
    }
    iVar4 = FUN_00622440(param_2,param_3,(LPWSTR)&DAT_006bec30,4,0);
    if (iVar4 != 0) {
      pbVar6 = &DAT_006bec30;
      *param_4 = '\0';
      while( true ) {
        bVar1 = *pbVar6;
        if (DAT_006af3ac < 2) {
          uVar5 = (byte)PTR_DAT_006af1a0[(uint)bVar1 * 2] & 4;
        }
        else {
          uVar5 = FUN_0061f330((uint)bVar1,4);
        }
        if (uVar5 == 0) break;
        pbVar6 = pbVar6 + 2;
        *param_4 = *param_4 * '\n' + bVar1 + -0x30;
        if (0x6bec37 < (int)pbVar6) {
          return 0;
        }
      }
      return 0;
    }
    return 0xffffffff;
  }
  _Source = local_80;
  bVar2 = false;
  _Count = FUN_00622570(param_2,param_3,local_80,0x80,0);
  if (_Count == 0) {
    DVar3 = GetLastError();
    if (((DVar3 != 0x7a) || (iVar4 = FUN_00622570(param_2,param_3,(LPSTR)0x0,0,0), iVar4 == 0)) ||
       (_Source = (LPSTR)FUN_00619170(iVar4), _Source == (LPSTR)0x0)) goto LAB_00626920;
    bVar2 = true;
    _Count = FUN_00622570(param_2,param_3,_Source,iVar4,0);
    if (_Count == 0) goto LAB_00626920;
  }
  _Dest = (char *)FUN_00619170(_Count);
  *(char **)param_4 = _Dest;
  if (_Dest != (char *)0x0) {
    _strncpy(_Dest,_Source,_Count);
    if (!bVar2) {
      return 0;
    }
    FUN_00618e60(_Source);
    return 0;
  }
LAB_00626920:
  if (!bVar2) {
    return 0xffffffff;
  }
  FUN_00618e60(_Source);
  return 0xffffffff;
}

