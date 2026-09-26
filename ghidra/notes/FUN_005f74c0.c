
void * __thiscall
FUN_005f74c0(void *this,char *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            int param_5,int param_6,undefined4 param_7,undefined4 param_8)

{
  char cVar1;
  HCURSOR pHVar2;
  HGDIOBJ pvVar3;
  uint uVar4;
  char *pcVar5;
  char *pcVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  uint uVar10;
  
  uVar10 = 0;
  uVar9 = 0;
  uVar8 = 0;
  pHVar2 = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f00);
  uVar7 = 0;
  pvVar3 = GetStockObject(4);
  FUN_005ff910(this,param_2,s_Video_Window_006ac278,0x50000000,param_3,param_4,param_5,param_6,
               param_7,(char *)0x0,pvVar3,uVar7,(int)pHVar2,uVar8,uVar9,uVar10);
  uVar10 = 0xffffffff;
  *(undefined ***)this = &PTR_FUN_0066ddf8;
  do {
    pcVar5 = param_1;
    if (uVar10 == 0) break;
    uVar10 = uVar10 - 1;
    pcVar5 = param_1 + 1;
    cVar1 = *param_1;
    param_1 = pcVar5;
  } while (cVar1 != '\0');
  uVar10 = ~uVar10;
  pcVar5 = pcVar5 + -uVar10;
  pcVar6 = (char *)((int)this + 0x94);
  for (uVar4 = uVar10 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar6 = pcVar6 + 4;
  }
  for (uVar10 = uVar10 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
    *pcVar6 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar6 = pcVar6 + 1;
  }
  *(undefined4 *)((int)this + 0x198) = 0;
  *(undefined4 *)((int)this + 0x19c) = 0;
  *(undefined4 *)((int)this + 0x1a0) = param_8;
  return this;
}

