
void __thiscall FUN_005f3430(void *this,int param_1,size_t param_2,int param_3)

{
  char *_Str1;
  byte *pbVar1;
  char *pcVar2;
  
  if (param_3 != 0) {
    pcVar2 = *(char **)(param_1 + 4);
    if (pcVar2 == (char *)0x0) {
      pcVar2 = &DAT_006b120c;
    }
    _Str1 = *(char **)((int)this + 4);
    if (_Str1 == (char *)0x0) {
      _Str1 = &DAT_006b120c;
    }
    _strncmp(_Str1,pcVar2,param_2);
    return;
  }
  pcVar2 = *(char **)(param_1 + 4);
  if (pcVar2 == (char *)0x0) {
    pcVar2 = &DAT_006b120c;
  }
  pbVar1 = *(byte **)((int)this + 4);
  if (pbVar1 == (byte *)0x0) {
    pbVar1 = &DAT_006b120c;
  }
  FUN_0061a620(pbVar1,pcVar2,param_2);
  return;
}

