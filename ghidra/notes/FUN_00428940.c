
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_00428940(HWND param_1,int *param_2,int *param_3)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  HDC hdc;
  HGDIOBJ pvVar6;
  HGDIOBJ pvVar7;
  int iVar8;
  int top;
  int iVar9;
  int iVar10;
  longlong lVar11;
  longlong lVar12;
  int local_2c;
  int local_28;
  int local_8;
  
  piVar2 = param_3;
  iVar4 = *param_2;
  iVar1 = param_2[1];
  local_8 = *param_3;
  top = param_3[1];
  iVar9 = iVar4 - local_8;
  if (iVar9 == 0) {
    param_3 = (int *)0x0;
  }
  else {
    param_3 = (int *)((float)(iVar1 - top) / (float)iVar9);
  }
  lVar11 = __ftol();
  iVar3 = iVar1 - top;
  iVar10 = piVar2[2] - local_8;
  iVar8 = piVar2[3] - top;
  iVar4 = (param_2[2] - iVar10) - iVar4;
  iVar5 = (param_2[3] - iVar8) - iVar1;
  hdc = GetDC(param_1);
  pvVar6 = GetStockObject(5);
  pvVar6 = SelectObject(hdc,pvVar6);
  pvVar7 = GetStockObject(6);
  pvVar7 = SelectObject(hdc,pvVar7);
  SetROP2(hdc,7);
  param_2 = (int *)(iVar8 + top);
  local_2c = iVar10 + local_8;
  Rectangle(hdc,local_8,top,local_2c,(int)param_2);
  local_28 = 4;
  do {
    Rectangle(hdc,local_8,top,local_2c,(int)param_2);
    local_8 = local_8 + ((int)(iVar9 + (iVar9 >> 0x1f & 3U)) >> 2);
    if ((float)param_3 == _DAT_00658bf0) {
      top = top + ((int)(iVar3 + (iVar3 >> 0x1f & 3U)) >> 2);
    }
    else {
      lVar12 = __ftol();
      top = (int)lVar12 + (iVar1 - (int)lVar11);
    }
    iVar8 = iVar8 + ((int)(iVar5 + (iVar5 >> 0x1f & 3U)) >> 2);
    iVar10 = iVar10 + ((int)(iVar4 + (iVar4 >> 0x1f & 3U)) >> 2);
    param_2 = (int *)(iVar8 + top);
    local_2c = iVar10 + local_8;
    Rectangle(hdc,local_8,top,local_2c,(int)param_2);
    SleepEx(0x32,0);
    local_28 = local_28 + -1;
  } while (local_28 != 0);
  Rectangle(hdc,local_8,top,iVar10 + local_8,iVar8 + top);
  SelectObject(hdc,pvVar6);
  SelectObject(hdc,pvVar7);
  ReleaseDC(param_1,hdc);
  return;
}

