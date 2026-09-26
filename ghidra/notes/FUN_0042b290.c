
void __thiscall FUN_0042b290(void *this,uint param_1,uint param_2,uint param_3)

{
  HMODULE hModule;
  HRSRC hResInfo;
  uint uVar1;
  uint uVar2;
  HGLOBAL hResData;
  undefined2 *puVar3;
  
  puVar3 = (undefined2 *)0x0;
  hResData = (HGLOBAL)0x0;
  hModule = (HMODULE)FUN_005fefd0(9);
  hResInfo = FindResourceA(hModule,(LPCSTR)(param_1 & 0xffff),(LPCSTR)0xa);
  if (hResInfo != (HRSRC)0x0) {
    hResData = LoadResource(hModule,hResInfo);
    if (hResData != (HGLOBAL)0x0) {
      puVar3 = LockResource(hResData);
    }
  }
  if ((puVar3 != (undefined2 *)0x0) && (*(int **)((int)this + 300) != (int *)0x0)) {
    uVar2 = param_2;
    if (param_2 == 0) {
      uVar2 = (uint)(ushort)puVar3[2];
    }
    uVar1 = param_3;
    if (param_3 == 0) {
      uVar1 = (uint)(ushort)puVar3[3];
    }
    param_1 = CONCAT22(9,puVar3[1]);
    (**(code **)(**(int **)((int)this + 300) + 0x38))(0xbeef,&param_1,*puVar3,uVar2,uVar1);
    FreeResource(hResData);
  }
  return;
}

