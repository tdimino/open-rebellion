
void __thiscall
FUN_0060b840(void *this,void *param_1,uint param_2,undefined4 *param_3,undefined4 *param_4,
            undefined4 *param_5,undefined4 *param_6)

{
  byte bVar1;
  bool bVar2;
  HRSRC hResInfo;
  HGLOBAL hResData;
  int iVar3;
  undefined4 *puVar4;
  char *_Str1;
  
  _Str1 = (char *)0x0;
  bVar2 = false;
  hResInfo = FindResourceA(*(HMODULE *)((int)this + 8),(LPCSTR)(param_2 & 0xffff),(LPCSTR)0xa);
  hResData = LoadResource(*(HMODULE *)((int)this + 8),hResInfo);
  if (hResData != (HGLOBAL)0x0) {
    _Str1 = LockResource(hResData);
  }
  if (_Str1 != (char *)0x0) {
    do {
      iVar3 = _strncmp(_Str1,(char *)((int)this + 4),1);
      if (iVar3 == 0) {
        bVar2 = true;
      }
      else {
        iVar3 = _strncmp(_Str1,(char *)((int)this + 5),1);
        if (iVar3 == 0) {
          bVar1 = _Str1[1];
          if ((bVar1 & 0x80) == 0) {
            switch((uint)bVar1) {
            case 1:
              puVar4 = param_3;
              break;
            case 2:
              puVar4 = param_4;
              break;
            case 3:
              puVar4 = param_5;
              break;
            case 4:
              puVar4 = param_6;
              break;
            default:
              puVar4 = (undefined4 *)0x0;
            }
          }
          else {
            puVar4 = *(undefined4 **)((int)this + (bVar1 & 0x7f) * 4);
          }
          if (puVar4 != (undefined4 *)0x0) {
            (**(code **)*puVar4)(param_1,*(undefined4 *)(_Str1 + 2));
          }
          _Str1 = _Str1 + 6;
        }
        else {
          FUN_005f3240(param_1,_Str1);
          _Str1 = _Str1 + 1;
        }
      }
    } while (!bVar2);
  }
  FreeResource(hResData);
  return;
}

